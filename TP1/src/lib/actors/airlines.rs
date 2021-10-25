// use std::{collections::HashMap, error::Error};

// use actix::{Actor, Addr, Context, Handler};

// use crate::actors::logger::{LogMessage, Logger};
// use crate::common::config::{AirlineConfig, AirlinesConfig};

// use super::request::SystemRequest;

// pub struct Airline {
//     name: String,
//     failure_rate: f64,
//     rate_limit: isize,
//     retry_time: u64,
//     logger_addr: Addr<Logger>,
// }

// impl Actor for Airline {
//     type Context = Context<Self>;
// }

// impl Handler<SystemRequest> for Airline {
//     type Result = ();

//     fn handle(&mut self, msg: SystemRequest, _: &mut Context<Self>) {
//         // OJO CON BLOQUEAR EL MAIN LOOP
//         self.logger_addr.do_send(LogMessage(format!(
//             "Hola soy la aerolinea: {} y tengo que ir de {} hasta {}. FR: {}, RL: {}, RT: {}",
//             self.name, msg.origin, msg.destiny, self.failure_rate, self.rate_limit, self.retry_time
//         )));
//     }
// }

// pub type Airlines = HashMap<String, Addr<Airline>>;

// pub fn from_path(path: &str, logger_addr: Addr<Logger>) -> Result<Airlines, Box<dyn Error>> {
//     let mut content = Airlines::new();

//     let data = std::fs::read_to_string(path)?;
//     let airlines: AirlinesConfig = serde_json::from_str(&data)?;

//     for AirlineConfig {
//         name,
//         rate_limit,
//         failure_rate,
//         retry_time,
//         min_delay,
//         max_delay,
//     } in airlines
//     {
//         content.insert(
//             name.clone(),
//             Airline {
//                 name,
//                 rate_limit,
//                 failure_rate,
//                 retry_time,
//                 logger_addr: logger_addr.clone(),
//                 min_delay,
//                 max_delay,
//             }
//             .start(),
//         );
//     }

//     Ok(content)
// }
