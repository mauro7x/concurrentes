use crate::{node::Node, protocol::Opcode, types::*};

// ----------------------------------------------------------------------------

pub fn next(id: Id) -> Id {
    (id + 1) % Id::MAX
}

pub fn encode(node: &Node) -> BoxResult<Vec<u8>> {
    let mut msg = vec![];
    msg.extend_from_slice(&node.id.to_le_bytes());
    msg.extend_from_slice(&node.ip.octets().to_vec());

    Ok(msg)
}

pub fn msg_from(opcode: Opcode, node: &Node) -> BoxResult<Vec<u8>> {
    let mut msg = Vec::from(opcode);
    let mut encoded_node = encode(node)?;
    msg.append(&mut encoded_node);

    Ok(msg)
}
