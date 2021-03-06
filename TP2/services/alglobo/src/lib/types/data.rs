use std::collections::HashMap;

use serde::Deserialize;

// ----------------------------------------------------------------------------

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    Prepare,
    Commit,
    Abort,
    Terminate, // end services
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
    pub tx: Transaction,
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Transaction {
    pub id: u32,
    pub cbu: u32,
    pub airline_cost: u32,
    pub hotel_cost: u32,
}

pub type Tx = u32;

pub type Responses = HashMap<Entity, Option<Action>>;
