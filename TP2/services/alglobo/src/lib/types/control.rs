use std::{
    collections::HashMap,
    net::Ipv4Addr,
    sync::{Condvar, Mutex},
};

use crate::types::common::Id;

// ----------------------------------------------------------------------------

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

pub struct Node {
    pub id: Id,
    pub ip: Ipv4Addr,
}
