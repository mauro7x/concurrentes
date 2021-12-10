use std::{
    fmt::Debug,
    net::{IpAddr, TcpStream},
};

use crate::types::*;

#[derive(Debug)]
pub struct Node {
    pub id: Id,
    pub ip: IpAddr,
    pub stream: TcpStream,
}
