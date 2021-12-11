use std::{collections::HashMap, error::Error, net::Ipv4Addr};

// ----------------------------------------------------------------------------

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

pub type Id = u8;

pub struct Node {
    pub id: Id,
    pub ip: Ipv4Addr,
}

pub type Ip2Id = HashMap<Ipv4Addr, Id>;
pub type Id2Ip = HashMap<Id, Ipv4Addr>;
