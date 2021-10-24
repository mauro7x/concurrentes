use actix::{Actor, Addr, AsyncContext, Context, Handler, Message};

const PACKAGE_TASKS: u64 = 2;
const NON_PACKAGE_TASKS: u64 = 1;

enum status {
    IN_PROGRESS,
    DONE,
}

pub struct RequestStatus {
    req: Request,
    pendingTasks: u64,
}

pub struct StatusService {
    reqs: HashMap<RequestStatus>
}

impl RequestStatus {
    pub fn new(req: Request) -> Self {
        RequestStatus {
            req,
            pendingTasks: if req.package { PACKAGE_TASKS } else { NON_PACKAGE_TASKS },
        }
    }
}
