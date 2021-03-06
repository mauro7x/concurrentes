//! Metrics collector entities.
//!
//! Metrics collected:
//! * Mean request time
//! * Top n routes
//! * Number of requests

use std::{
    collections::HashMap,
    error::Error,
    sync::mpsc::{channel, Receiver, Sender},
    sync::{Arc, RwLock},
    thread::{self, spawn, JoinHandle},
    time,
};

use crate::common::{config::MetricsCollectorConfig, utils};
use crate::part1::request::{Request, RequestDuration};

struct Metrics {
    routes_booking_count: HashMap<(String, String), u64>,
    reqs_duration_cumsum: i64,
    n_reqs: u64,
}

/// MetricsCollector is an entity that keeps a reference to the threads
/// that handle the metrics. One of the threads will be in charge of collecting
/// metrics using a channel for communication, the other will be in charge of printing
/// to stdout periodically the metrics collected. Both threads get syncronized using a RwLock.

pub struct MetricsCollector {
    collector_handler: JoinHandle<()>,
    keep_running: Arc<RwLock<bool>>,
    printer_handler: JoinHandle<()>,
    tx: Sender<RequestDuration>,
}

/// MetricsSender holds a reference to the channel that handles the communication
/// with the metrics collector thread.

#[derive(Clone)]
pub struct MetricsSender {
    tx: Sender<RequestDuration>,
}

impl MetricsCollector {
    /// Given a MetricsCollectorConfig this method will create a MetricsCollector entity,
    /// and spawn the corresponding threads.

    pub fn from_config(config: MetricsCollectorConfig) -> Result<MetricsCollector, Box<dyn Error>> {
        let (tx, rx): (Sender<RequestDuration>, Receiver<RequestDuration>) = channel();
        let keep_running = Arc::new(RwLock::new(true));
        let metrics = Arc::new(RwLock::new(Metrics {
            routes_booking_count: HashMap::new(),
            reqs_duration_cumsum: 0,
            n_reqs: 0,
        }));

        let collector_metrics = metrics.clone();
        let printer_keep_running = keep_running.clone();

        let collector_handler =
            spawn(move || MetricsCollector::collect_metrics(rx, collector_metrics));
        let printer_handler = spawn(move || {
            MetricsCollector::print_metrics_periodically(
                metrics,
                printer_keep_running,
                time::Duration::from_millis(config.printer_period),
                config.n_most_booked,
            )
        });

        let metrics_collector = MetricsCollector {
            collector_handler,
            keep_running,
            printer_handler,
            tx,
        };

        Ok(metrics_collector)
    }

    fn collect_metrics(rx: Receiver<RequestDuration>, metrics: Arc<RwLock<Metrics>>) {
        while let Ok((req, time)) = rx.recv() {
            MetricsCollector::compute_request(req, time, &metrics);
        }
    }

    fn compute_request(req: Request, time: i64, metrics_lock: &Arc<RwLock<Metrics>>) {
        let route_key = (req.origin, req.destiny);
        let mut metrics = metrics_lock
            .write()
            .expect("[CRITICAL] Could not take metrics write lock");

        metrics.n_reqs += 1;
        metrics.reqs_duration_cumsum += time;

        if let Some(route_count) = metrics.routes_booking_count.get_mut(&route_key) {
            *route_count += 1;
        } else {
            metrics.routes_booking_count.insert(route_key, 1);
        }
    }

    fn get_n_most_booked_routes(
        routes_booking_count: &HashMap<(String, String), u64>,
        n: usize,
    ) -> Vec<(&(std::string::String, std::string::String), &u64)> {
        let mut routes_booking_count_vec: Vec<_> = routes_booking_count.iter().collect();
        routes_booking_count_vec.sort_by(|a, b| b.1.cmp(a.1));
        routes_booking_count_vec.truncate(n);
        routes_booking_count_vec
    }

    fn print_metrics(metrics_lock: &Arc<RwLock<Metrics>>, n: usize) {
        let metrics = metrics_lock
            .read()
            .expect("[CRITICAL] Printer could not read metrics lock");

        let n_reqs = metrics.n_reqs;
        let routes_by_bookings =
            MetricsCollector::get_n_most_booked_routes(&metrics.routes_booking_count, n);

        let mut most_booked_routes_msg: String = format!(
            "{:=^36}\n|{:^4}|{:^9}|{:^9}|{:^9}|\n{:=^36}",
            "", "N??", "ORIGIN", "DESTINY", "#", ""
        );

        for (i, route) in routes_by_bookings.iter().enumerate() {
            let ((origin, destiny), amount) = route;

            most_booked_routes_msg += &format!(
                "\n|{:^4}|{:^9}|{:^9}|{:^9}|",
                i + 1,
                origin,
                destiny,
                amount
            );
        }
        most_booked_routes_msg += &format!("\n{:=^36}", "");

        println!(
            "[{}] Requests successfully processed: {} reqs",
            utils::now_h_m_s(),
            n_reqs
        );

        if n_reqs > 0 {
            println!(
                "[{}] Mean time to book: {} ms",
                utils::now_h_m_s(),
                metrics.reqs_duration_cumsum / (n_reqs as i64)
            );
            println!(
                "[{}] Most booked routes:\n{}",
                utils::now_h_m_s(),
                most_booked_routes_msg
            );
        };
    }

    fn print_metrics_periodically(
        metrics_lock: Arc<RwLock<Metrics>>,
        keep_running_lock: Arc<RwLock<bool>>,
        period: std::time::Duration,
        n_most_booked: usize,
    ) {
        loop {
            {
                let keep_running = keep_running_lock
                    .read()
                    .expect("[CRITICAL] Printer could not read keep_running lock");
                if !(*keep_running) {
                    break;
                };
            }
            MetricsCollector::print_metrics(&metrics_lock, n_most_booked);
            thread::sleep(period);
        }
        MetricsCollector::print_metrics(&metrics_lock, n_most_booked);
    }

    pub fn get_sender(&self) -> MetricsSender {
        MetricsSender {
            tx: self.tx.clone(),
        }
    }

    pub fn join(self) {
        drop(self.tx);
        self.collector_handler
            .join()
            .expect("[CRITICAL] Error joining metrics collector thread");
        {
            let mut keep_running_value = self
                .keep_running
                .write()
                .expect("[CRITICAL] Joiner could not write keep_running lock");
            *keep_running_value = false;
        }
        self.printer_handler
            .join()
            .expect("[CRITICAL] Error joining metrics printer thread");
    }
}

impl MetricsSender {
    pub fn send(&self, req: Request, duration_ms: i64) {
        let _ = self.tx.send((req, duration_ms));
    }
}
