use std::{
    convert::TryInto,
    net::{SocketAddr, UdpSocket},
};

use crate::types::{
    common::BoxResult,
    data::{Action, Entity, Message},
};

// ----------------------------------------------------------------------------

// Message structure
//
// | --- 1 byte --- | --- 1 byte --- | -- 4 bytes -- |
// |   FROM_ENTITY  |     ACTION     |   TX_NUMBER   |
// | ---------------|----------------|---------------|

// FROM_ENTITIES
const AIRLINE_REP: u8 = b'A';
const BANK_REP: u8 = b'B';
const ALGLOBO_REP: u8 = b'C';
const HOTEL_REP: u8 = b'D';

// ACTIONS
const COMMIT_REP: u8 = b'E';
const PREPARE_REP: u8 = b'F';
const ABORT_REP: u8 = b'G';

// ----------------------------------------------------------------------------
// Public

pub fn send_msg_to(socket: &UdpSocket, msg: &Message, addr: &SocketAddr) -> BoxResult<usize> {
    let buf = pack_message(msg);
    let sent = socket.send_to(&buf, addr)?;

    Ok(sent)
}

pub fn recv_msg(socket: &UdpSocket) -> BoxResult<(SocketAddr, Message)> {
    let mut buf = vec![0; 6];
    let (_, from) = socket.recv_from(&mut buf)?;
    let msg = unpack_message(&buf)?;

    Ok((from, msg))
}

// ----------------------------------------------------------------------------
// Private
pub fn cast_action(action: &Action) -> u8 {
    match action {
        Action::Prepare => PREPARE_REP,
        Action::Commit => COMMIT_REP,
        Action::Abort => ABORT_REP,
    }
}

fn pack_message(msg: &Message) -> Vec<u8> {
    let mut buf = vec![];

    let from_rep = match &msg.from {
        Entity::Airline => AIRLINE_REP,
        Entity::AlGlobo => ALGLOBO_REP,
        Entity::Bank => BANK_REP,
        Entity::Hotel => HOTEL_REP,
    };

    buf.push(from_rep);

    let action_rep = cast_action(&msg.action);
    buf.push(action_rep);

    buf.append(&mut msg.tx.to_le_bytes().to_vec());

    buf
}

fn unpack_message(buf: &[u8]) -> BoxResult<Message> {
    let from = match buf[0] {
        AIRLINE_REP => Entity::Airline,
        ALGLOBO_REP => Entity::AlGlobo,
        BANK_REP => Entity::Bank,
        HOTEL_REP => Entity::Hotel,
        _ => return Err(format!("Unknown message from entity {}", buf[0]).into()),
    };

    let action = match buf[1] {
        COMMIT_REP => Action::Commit,
        PREPARE_REP => Action::Prepare,
        ABORT_REP => Action::Abort,
        _ => return Err(format!("Unknown action ({}) from entity {}", buf[1], buf[0]).into()),
    };

    let tx = u32::from_le_bytes(buf[2..6].try_into()?);

    Ok(Message { from, action, tx })
}
