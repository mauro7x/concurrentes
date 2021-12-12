use std::{
    fmt::Debug,
    net::{Ipv4Addr, TcpStream},
};

use crate::types::*;

#[derive(Debug)]
pub struct Node {
    pub id: Id,
    pub ip: Ipv4Addr,
    pub stream: TcpStream,
}
