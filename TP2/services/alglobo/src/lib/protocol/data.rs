use crate::types::{Action, Entity, Message};
use std::{convert::TryInto, net::UdpSocket};

// Message structure
//
// | --- 1 byte --- | --- 1 byte --- | -- 4 bytes -- |
// |   FROM_ENTITY  |     ACTION     |   TX_NUMBER   |
// | ---------------|----------------|---------------|
//

// FROM_ENTITIES
const AIRLINE_REP: u8 = b'A';
const BANK_REP: u8 = b'B';
const ALGLOBO_REP: u8 = b'C';
const HOTEL_REP: u8 = b'D';

// ACTIONS
const COMMIT_REP: u8 = b'E';
const PREPARE_REP: u8 = b'F';
const ABORT_REP: u8 = b'G';

pub fn pack_message(msg: &Message, buf: &mut Vec<u8>) {
    let from_rep = match &msg.from {
        Entity::Airline => AIRLINE_REP,
        Entity::AlGlobo => ALGLOBO_REP,
        Entity::Bank => BANK_REP,
        Entity::Hotel => HOTEL_REP,
    };

    buf.push(from_rep);

    let action_rep = match &msg.action {
        Action::Prepare => PREPARE_REP,
        Action::Commit => COMMIT_REP,
        Action::Abort => ABORT_REP,
    };

    buf.push(action_rep);

    buf.append(&mut msg.tx.to_le_bytes().to_vec());
}

fn unpack_message(buf: &Vec<u8>) -> Message {
    let from = match buf[0] {
        AIRLINE_REP => Entity::Airline,
        ALGLOBO_REP => Entity::AlGlobo,
        BANK_REP => Entity::Bank,
        HOTEL_REP => Entity::Hotel,
        _ => panic!("unpack_message: unknown from entity {}", buf[0]),
    };

    let action = match buf[1] {
        COMMIT_REP => Action::Commit,
        PREPARE_REP => Action::Prepare,
        ABORT_REP => Action::Abort,
        _ => panic!("unpack_message: unknown action {}", buf[1]),
    };

    let tx = u32::from_le_bytes(
        buf[2..6]
            .try_into()
            .expect("unpack_message: cannot unpack tx"),
    );

    Message { from, action, tx }
}

pub fn recv_msg(socket: &mut UdpSocket) -> std::io::Result<(String, Message)> {
    let mut buf = vec![0; 6];
    let (_, src) = socket.recv_from(&mut buf)?;
    let msg = unpack_message(&buf);
    Ok((src.to_string(), msg))
}
