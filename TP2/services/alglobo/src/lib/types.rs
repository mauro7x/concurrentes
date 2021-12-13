use std::{
    collections::HashMap,
    error::Error,
    net::Ipv4Addr,
    sync::{Condvar, Mutex},
};

use serde::Deserialize;

// ----------------------------------------------------------------------------

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

pub type Id = u8;

pub struct Node {
    pub id: Id,
    pub ip: Ipv4Addr,
}

pub type Ip2Id = HashMap<Ipv4Addr, Id>;
pub type Id2Ip = HashMap<Id, Ipv4Addr>;

pub struct Shared<T> {
    pub mutex: Mutex<T>,
    pub cv: Condvar,
}

impl<T> Shared<T> {
    pub fn new(t: T) -> Self {
        Self {
            mutex: Mutex::new(t),
            cv: Condvar::new(),
        }
    }
}

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
