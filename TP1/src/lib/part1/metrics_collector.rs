use crate::common::{
    config::MetricsCollectorConfig,
    request::{Request, RequestDuration},
};

use std::{
    collections::HashMap,
    error::Error,
    sync::mpsc::{channel, Receiver, Sender},
    sync::{Arc, RwLock},
    thread::{self, spawn, JoinHandle},
    time,
};

struct Metrics {
    routes_booking_count: HashMap<(String, String), u64>,
    reqs_duration_cumsum: i64,
    n_reqs: u64,
}

pub struct MetricsCollector {
    collector_handler: JoinHandle<()>,
    keep_running: Arc<RwLock<bool>>,
    printer_handler: JoinHandle<()>,
    tx: Sender<RequestDuration>,
}

#[derive(Clone)]
pub struct MetricsSender {
    tx: Sender<RequestDuration>,
}

impl MetricsCollector {
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
        let mut most_booked_routes_msg: String = "".to_owned();
        let metrics = metrics_lock
            .read()
            .expect("[CRITICAL] Printer could not read metrics lock");
        let routes_by_bookings =
            MetricsCollector::get_n_most_booked_routes(&metrics.routes_booking_count, n);

        for route in routes_by_bookings {
            most_booked_routes_msg += &format!("- {}/{} ({}) ", route.0 .0, route.0 .1, route.1);
        }

        println!(
            "[METRICS] Requests successfully processed: {} reqs",
            metrics.n_reqs
        );
        if metrics.n_reqs != 0 {
            println!(
                "[METRICS] Mean time to book: {} ms",
                metrics.reqs_duration_cumsum / (metrics.n_reqs as i64)
            );
            println!("[METRICS] Most booked routes: {}", most_booked_routes_msg);
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
