//! Metrics collector entities.
//!
//! Metrics collected:
//! * Mean request time
//! * Top n routes
//! * Number of requests

use std::collections::HashMap;
use std::time::Duration;

use actix::{
    clock::sleep, Actor, ActorFutureExt, Addr, AsyncContext, Context, Handler, Message,
    ResponseActFuture, WrapFuture,
};
use actix_web::Result;
use serde::Serialize;

use crate::common::config::MetricsCollectorConfig;
use crate::part2::logger::Logger;

// TYPES ----------------------------------------------------------------------

/// Struct that is used as hash index to keep track of metrics for that route.
/// It has origin and destiny for request petition.
#[derive(Serialize, Clone, PartialEq, Eq, Hash)]
pub struct Route {
    origin: String,
    destiny: String,
}

/// Struct that is used to output top booked airline metrics.
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

/// MetricsCollector is an entity <Actor>. It will be in charge of collecting
/// metrics using MetricsMessage. It will also log periodically to stdout metrics status using inner calls.

pub struct MetricsCollector {
    metrics: Metrics,
    printer_period: u64,
    n_most_booked: usize,
    logger_addr: Addr<Logger>,
}

impl MetricsCollector {
    /// Given a MetricsCollectorConfig this method will create a MetricsCollector entity.

    pub fn new(
        MetricsCollectorConfig {
            printer_period,
            n_most_booked,
        }: MetricsCollectorConfig,
        logger_addr: Addr<Logger>,
    ) -> Self {
        MetricsCollector {
            metrics: Metrics {
                routes_booking_count: HashMap::new(),
                reqs_duration_cumsum: 0,
                n_reqs: 0,
            },
            printer_period,
            n_most_booked,
            logger_addr,
        }
    }

    /// Given a MetricsCollector addr this method is used to send the actor current petition metrics.

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
        let n = self.n_most_booked;

        let mut routes_booking_count_vec: Vec<(&Route, &u64)> =
            routes_booking_count.iter().collect();
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

    fn log_metrics(&self) {
        let n_reqs = self.metrics.n_reqs;
        let most_booked_routes = self.get_n_most_booked_routes();

        let mut most_booked_routes_msg: String = format!(
            "{:=^36}\n|{:^4}|{:^9}|{:^9}|{:^9}|\n{:=^36}",
            "", "Nº", "ORIGIN", "DESTINY", "#", ""
        );

        for (
            i,
            RouteMetrics {
                route: Route { origin, destiny },
                amount,
            },
        ) in most_booked_routes.iter().enumerate()
        {
            most_booked_routes_msg += &format!(
                "\n|{:^4}|{:^9}|{:^9}|{:^9}|",
                i + 1,
                origin,
                destiny,
                amount
            );
        }
        most_booked_routes_msg += &format!("\n{:=^36}", "");

        Logger::send_to(
            &self.logger_addr,
            format!("Requests successfully processed: {} reqs", n_reqs),
        );
        if n_reqs > 0 {
            Logger::send_to(
                &self.logger_addr,
                format!(
                    "Mean time to book: {} ms",
                    self.metrics.reqs_duration_cumsum / (n_reqs as i64)
                ),
            );
            Logger::send_to(
                &self.logger_addr,
                format!("Most booked routes:\n{}", most_booked_routes_msg),
            );
        };
    }
}

impl Actor for MetricsCollector {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        Logger::send_to(&self.logger_addr, "[MetricsCollector] Started".to_string());
        ctx.address()
            .try_send(LogMetrics {})
            .expect("[CRITICAL] Could not auto-send LogMetrics msg to MetricsCollector");
    }
}

// MESSAGES -------------------------------------------------------------------

/// Message
#[derive(Message)]
#[rtype(result = "()")]
struct LogMetrics;

/// Message to provide Metrics for a petition.
#[derive(Message)]
#[rtype(result = "()")]
pub struct MetricsMessage {
    start_time: i64,
    end_time: i64,
    origin: String,
    destiny: String,
}

/// Response for GetMetrics message. It provides current status of those metrics.
#[derive(Message, Serialize)]
#[rtype(result = "()")]
pub struct MetricsResponse {
    pub n_req: u64,
    pub req_mean_time: i64,
    pub most_booked_routes: MostBookedRoutes,
}

/// GetMetrics message to get current status metrics.
#[derive(Message)]
#[rtype(result = "Result<MetricsResponse, ()>")]
pub struct GetMetrics;

// HANDLERS -------------------------------------------------------------------

impl Handler<LogMetrics> for MetricsCollector {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: LogMetrics, _ctx: &mut Context<Self>) -> Self::Result {
        self.log_metrics();

        // Loop with printer_period time
        Box::pin(
            sleep(Duration::from_millis(self.printer_period))
                .into_actor(self)
                .map(move |_result, _me, ctx| {
                    ctx.address().try_send(LogMetrics {}).expect(
                        "[CRITICAL] Could not auto-send LogMetrics msg to MetricsCollector",
                    );
                }),
        )
    }
}

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
