use std::collections::HashMap;

use actix::{Actor, Addr, Context, Handler, Message};
use actix_web::Result;
use serde::Serialize;

use crate::common::config::MetricsCollectorConfig;
use crate::part2::logger::Logger;

// TYPES ----------------------------------------------------------------------

#[derive(Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Route {
    origin: String,
    destiny: String,
}

#[derive(Serialize)]
pub struct RouteMetrics {
    route: Route,
    amount: u64,
}

pub type MostBookedRoutes = Vec<RouteMetrics>;

// ACTOR ----------------------------------------------------------------------

struct Metrics {
    routes_booking_count: HashMap<Route, u64>,
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

    fn get_n_most_booked_routes(&self) -> MostBookedRoutes {
        let routes_booking_count = &self.metrics.routes_booking_count;
        let n = self.config.n_most_booked;

        let mut routes_booking_count_vec: Vec<_> = routes_booking_count.iter().collect();
        routes_booking_count_vec.sort_by(|a, b| b.1.cmp(a.1));
        routes_booking_count_vec.truncate(n);

        routes_booking_count_vec
            .into_iter()
            .map(|(route, amount)| RouteMetrics {
                route: route.clone(),
                amount: *amount,
            })
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

#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct MetricsResponse {
    pub n_req: u64,
    pub req_mean_time: i64,
    pub most_booked_routes: MostBookedRoutes,
}

#[derive(Message)]
#[rtype(result = "Result<MetricsResponse, ()>")]
pub struct GetMetrics;

// HANDLERS -------------------------------------------------------------------

impl Handler<MetricsMessage> for MetricsCollector {
    type Result = ();

    fn handle(
        &mut self,
        MetricsMessage {
            start_time,
            end_time,
            origin,
            destiny,
        }: MetricsMessage,
        _ctx: &mut Context<Self>,
    ) {
        let time = end_time - start_time;

        let route_key = Route { origin, destiny };

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
            "[MetricsCollector] Metrics request received".to_string(),
        );
        let mut req_mean_time = 0;

        if self.metrics.n_reqs > 0 {
            req_mean_time = self.metrics.reqs_duration_cumsum / (self.metrics.n_reqs as i64);
        }

        let most_booked_routes = self.get_n_most_booked_routes();

        Ok(MetricsResponse {
            req_mean_time,
            most_booked_routes,
            n_req: self.metrics.n_reqs,
        })
    }
}
