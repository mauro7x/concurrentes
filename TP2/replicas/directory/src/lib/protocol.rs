use std::{error::Error, net::IpAddr};

use crate::node::Node;

// Receive messages
pub type RecvMessage = [u8; 1];
pub const EMPTY_MESSAGE: RecvMessage = [0];
pub const REGISTER: RecvMessage = [b'R'];
pub const FINISHED: RecvMessage = [b'F'];

// Send messages
pub const ACCEPTED: u8 = b'A';
pub const REJECTED: u8 = b'R';
pub const EOB: u8 = b'E';
pub const PING: u8 = b'P';
// --- broadcast msgs
pub const NEW: u8 = b'N';
pub const DEAD: u8 = b'D';

// Helpers
pub fn encode(node: &Node) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut msg = vec![];
    msg.extend_from_slice(&node.id.to_le_bytes());
    msg.extend_from_slice(&[b'=']);

    let encoded_ip = match node.ip {
        IpAddr::V4(ip) => Ok(ip.octets().to_vec()),
        _ => Err("Not valid IPv4"),
    }?;
    msg.extend_from_slice(&encoded_ip);

    Ok(msg)
}

pub fn msg_from(header: u8, node: &Node) -> Result<Vec<u8>, Box<dyn Error>> {
    let mut msg = vec![header];
    let mut encoded_node = encode(node)?;
    msg.append(&mut encoded_node);

    Ok(msg)
}
