use crate::{node::Node, types::BoxResult};

// ----------------------------------------------------------------------------

pub type SendOpcode = [u8; 1];
pub const ACCEPTED: SendOpcode = [b'A'];
pub const REJECTED: SendOpcode = [b'R'];
pub const EOB: SendOpcode = [b'E'];
pub const PING: SendOpcode = [b'P'];
pub const NEW: SendOpcode = [b'N'];
pub const DEAD: SendOpcode = [b'D'];

pub type RecvOpcode = [u8; 1];
pub const REGISTER: RecvOpcode = [b'R'];
pub const FINISHED: RecvOpcode = [b'F'];

// ----------------------------------------------------------------------------

pub fn encode(node: &Node) -> BoxResult<Vec<u8>> {
    let mut msg = vec![];
    msg.extend_from_slice(&node.id.to_le_bytes());
    msg.extend_from_slice(&node.ip.octets().to_vec());

    Ok(msg)
}

pub fn msg_from(opcode: SendOpcode, node: &Node) -> BoxResult<Vec<u8>> {
    let mut msg = Vec::from(opcode);
    let mut encoded_node = encode(node)?;
    msg.append(&mut encoded_node);

    Ok(msg)
}
