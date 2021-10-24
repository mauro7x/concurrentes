use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};

use crate::part2::{
    dispatcher::{BookFailed, BookSucceeded, WebServiceDispatcher},
    fetch,
};
pub struct WebService {
    pub name: String,
    failure_rate: f64,
}

impl WebService {
    pub fn new(name: String, failure_rate: f64) -> Self {
        WebService { name, failure_rate }
    }

    // async fn fetch(&mut self, msg: Book, _ctx: &mut Context<Self>) {
    //     match _ctx.address().send(Fetch {}).await {
    //         Ok(_) => {
    //             println!(
    //                 "[{} WebService] REQ {} Fetch succeeded...",
    //                 self.name, msg.req_id
    //             );
    //             msg.requester.try_send(BookSucceeded { req_id: msg.req_id });
    //         },
    //         Err(_) => {
    //             println!(
    //                 "[{} WebService] REQ {} Fetch failed...",
    //                 self.name, msg.req_id
    //             );
    //             msg.requester.try_send(BookFailed { req_id: msg.req_id });
    //         },
    //     }
    // }
}

impl Actor for WebService {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("[WebService] {} started", self.name);
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct Book {
    pub req_id: u64,
    pub requester: Addr<WebServiceDispatcher>,
}

impl Handler<Book> for WebService {
    type Result = ();

    fn handle(&mut self, msg: Book, _ctx: &mut Context<Self>) {
        // OJO CON BLOQUEAR EL MAIN LOOP
        match fetch::simulate_fetch(self.failure_rate) {
            Ok(_) => {
                println!(
                    "[{} WebService] REQ {} Fetch succeeded...",
                    self.name, msg.req_id
                );
                msg.requester.try_send(BookSucceeded { req_id: msg.req_id });
            }
            Err(_) => {
                println!(
                    "[{} WebService] REQ {} Fetch failed...",
                    self.name, msg.req_id
                );
                msg.requester.try_send(BookFailed { req_id: msg.req_id });
            }
        }
    }
}

// #[derive(Message)]
// #[rtype(result = "Result<(), ()>")]
// struct Fetch;

// impl Handler<Fetch> for WebService {
//     type Result = ResponseActFuture<Self, Result<(), ()>>;

//     fn handle(&mut self, msg: Fetch, _ctx: &mut <WebService as Actor>::Context) -> Self::Result  {
//         let mut rng = rand::thread_rng();

//         // Simulate fetch
//         let fetch_time = rng.gen_range(1..20);
//         println!("[{}] durmiendo por {}", self.name, fetch_time);
//         Box::pin(sleep(Duration::from_secs(fetch_time))
//             .into_actor(self)
//             .map(move |_result, me, _ctx| {
//                 println!("[{}] desperté", me.name);
//                 // Simulate status
//                 let coin = rng.gen_range(0.0..1.0);
//                 match coin > self.failure_rate {
//                     true => Ok(()),
//                     false => Err(()),
//                 }
//             }))
//     }
// }
