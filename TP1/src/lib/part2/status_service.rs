use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};
use crate::part2::request::Request;

const PACKAGE_TASKS: u64 = 2;
const NON_PACKAGE_TASKS: u64 = 1;

pub struct RequestStatus {
    req: Request,
    pendingTasks: u64,
}

impl RequestStatus {
    pub fn new(req: Request) -> Self {
        RequestStatus {
            req,
            pendingTasks: if req.package {
                PACKAGE_TASKS
            } else {
                NON_PACKAGE_TASKS
            },
        }
    }
}

// ACTOR ----------------------------------------------------------------------

pub struct StatusService {
    reqs: HashMap<String, RequestStatus>,
}

impl StatusService {
    pub fn new() -> Self {
        StatusService{
            reqs: HashMap<String, RequestStatus>::new(),
        }
    }
}

impl Actor for StatusService {
    type Context = Context<Self>;

    fn started(&mut self, _: &mut Self::Context) {
        println!("StatusService started");
    }
}

// MESSAGES -------------------------------------------------------------------

#[derive(Message)]
#[rtype(result = "()")]
pub struct NewRequest {
    pub req: Request,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BookSucceeded {
    pub req: Request,
}

// HANDLERS -------------------------------------------------------------------

impl Handler<NewRequest> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: NewRequest, ctx: &mut Context<Self>) {
        println!("[StatusService] Registering request {}", msg.req.id);
        self.reqs.insert(req.id, RequestStatus{ req });
    }
}

impl Handler<NewRequest> for WebServiceDispatcher {
    type Result = ();

    fn handle(&mut self, msg: NewRequest, ctx: &mut Context<Self>) {
        println!("[StatusService] Registering request {}", msg.req.id);
        self.reqs.insert(req.id, RequestStatus{ req });
    }
}

