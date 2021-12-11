#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Prepare,
    Commit,
    Abort
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Entity {
    Airline,
    AlGlobo,
    Bank,
    Hotel
}

#[derive(Debug)]
pub struct Message {
    pub from: Entity,
    pub action: Action,
    pub tx: u32
}

#[derive(Debug)]
pub enum Status {
    Failed,
    Succeeded
}

pub type Tx = u32;
