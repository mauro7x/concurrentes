use std::mem::size_of;

use crate::types::Id;

// ----------------------------------------------------------------------------

pub type Opcode = u8;
pub const OK: Opcode = b'O';
pub const ELECTION: Opcode = b'E';
pub const COORDINATOR: Opcode = b'C';
pub const PING: Opcode = b'P';

const MESSAGE_SIZE: usize = size_of::<Opcode>() + size_of::<Id>();
pub type Message = [u8; MESSAGE_SIZE];
pub const NEW_MESSAGE: Message = [0; MESSAGE_SIZE];
pub const PONG: [Opcode; 1] = [OK; 1];
