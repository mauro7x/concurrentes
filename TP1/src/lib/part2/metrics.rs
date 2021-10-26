use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};
use actix_web::Result;

use crate::common::config::MetricsCollectorConfig;
use crate::part2::logger::Logger;

// ACTOR ----------------------------------------------------------------------
struct Metrics {
    routes_booking_count: HashMap<(String, String), u64>,
    reqs_duration_cumsum: i64,
    n_reqs: u64,
}

pub struct MetricsCollector {
    metrics: Metrics,
    config: MetricsCollectorConfig,
    logger_addr: Addr<Logger>,
}

impl MetricsCollector {
    pub fn new(config: MetricsCollectorConfig, logger_addr: Addr<Logger>) -> Self {
        MetricsCollector {
            metrics: Metrics {
                routes_booking_count: HashMap::new(),
                reqs_duration_cumsum: 0,
                n_reqs: 0,
            },
            config,
            logger_addr,
        }
    }
    pub fn collect(
        metrics_collector: &Addr<MetricsCollector>,
        start_time: i64,
        end_time: i64,
        origin: String,
        destiny: String,
    ) {
        if metrics_collector
            .try_send(MetricsMessage {
                start_time,
                end_time,
                origin,
                destiny,
            })
            .is_err()
        {
            println!("Warning: failed to send metrics to MetricsMessage");
        };
    }
    fn get_n_most_booked_routes(
        routes_booking_count: &HashMap<(String, String), u64>,
        n: usize,
    ) -> Vec<((std::string::String, std::string::String), u64)> {
        let mut routes_booking_count_vec: Vec<_> = routes_booking_count.iter().collect();
        routes_booking_count_vec.sort_by(|a, b| b.1.cmp(a.1));
        routes_booking_count_vec.truncate(n);

        routes_booking_count_vec
            .into_iter()
            .map(|(route, req)| (route.clone(), *req))
            .collect()
    }
}

impl Actor for MetricsCollector {
    type Context = Context<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        Logger::send_to(&self.logger_addr, "[MetricsCollector] Started".to_string());
    }
}

// MESSAGES -------------------------------------------------------------------

#[derive(Message)]
#[rtype(result = "()")]

pub struct MetricsMessage {
    start_time: i64,
    end_time: i64,
    origin: String,
    destiny: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct MetricsResponse {
    pub n_req: u64,
    pub req_mean_time: i64,
    pub most_visited_routes: Vec<((std::string::String, std::string::String), u64)>,
}

#[derive(Message)]
#[rtype(result = "Result<MetricsResponse, ()>")]
pub struct GetMetrics;

// HANDLERS -------------------------------------------------------------------

impl Handler<MetricsMessage> for MetricsCollector {
    type Result = ();

    fn handle(&mut self, msg: MetricsMessage, _ctx: &mut Context<Self>) {
        let time = msg.end_time - msg.start_time;

        let route_key = (msg.origin, msg.destiny);

        self.metrics.n_reqs += 1;
        self.metrics.reqs_duration_cumsum += time;

        if let Some(route_count) = self.metrics.routes_booking_count.get_mut(&route_key) {
            *route_count += 1;
        } else {
            self.metrics.routes_booking_count.insert(route_key, 1);
        }
    }
}

impl Handler<GetMetrics> for MetricsCollector {
    type Result = Result<MetricsResponse, ()>;

    fn handle(
        &mut self,
        _msg: GetMetrics,
        _ctx: &mut Context<Self>,
    ) -> Result<MetricsResponse, ()> {
        Logger::send_to(
            &self.logger_addr,
            "[MetricsCollector] Metrics request".to_string(),
        );
        let mut req_mean_time = 0;

        if self.metrics.n_reqs > 0 {
            req_mean_time = self.metrics.reqs_duration_cumsum / (self.metrics.n_reqs as i64);
        }

        let most_visited_routes = MetricsCollector::get_n_most_booked_routes(
            &self.metrics.routes_booking_count,
            self.config.n_most_booked,
        );

        Ok(MetricsResponse {
            req_mean_time,
            most_visited_routes,
            n_req: self.metrics.n_reqs,
        })
    }
}
