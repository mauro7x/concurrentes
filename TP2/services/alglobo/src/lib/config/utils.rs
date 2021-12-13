use std::net::{SocketAddr, ToSocketAddrs};

use crate::types::common::BoxResult;

// ----------------------------------------------------------------------------

pub fn to_socket_addr(host: String, port: u16) -> BoxResult<SocketAddr> {
    let dns_query = format!("{}:{}", host, port);

    let addr = dns_query
        .to_socket_addrs()?
        .collect::<Vec<SocketAddr>>()
        .first()
        .ok_or("No IP address found for directory hostname")?
        .to_owned();

    Ok(addr)
}
