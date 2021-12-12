use serde::Deserialize;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Prepare,
    Commit,
    Abort,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Entity {
    Airline,
    AlGlobo,
    Bank,
    Hotel,
}

#[derive(Debug)]
pub struct Message {
    pub from: Entity,
    pub action: Action,
    pub tx: u32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub cbu: u32,
    pub bank_total: u32,
    pub airline_total: u32,
    pub hotel_total: u32,
}

pub type Tx = u32;
