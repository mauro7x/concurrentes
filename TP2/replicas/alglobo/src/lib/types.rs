use std::{error::Error, net::Ipv4Addr};

// ----------------------------------------------------------------------------

pub type BoxResult<T> = Result<T, Box<dyn Error>>;

pub type Id = u8;

pub struct Node {
    pub id: Id,
    pub addr: Ipv4Addr,
}
