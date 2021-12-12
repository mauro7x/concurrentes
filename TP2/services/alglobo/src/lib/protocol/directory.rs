pub type SendOpcode = [u8; 1];
pub const REGISTER: SendOpcode = [b'R'];
pub const FINISHED: SendOpcode = [b'F'];

pub type RecvOpcode = [u8; 1];
pub const ACCEPTED: RecvOpcode = [b'A'];
pub const REJECTED: RecvOpcode = [b'R'];
pub const EOB: RecvOpcode = [b'E'];
pub const NEW: RecvOpcode = [b'N'];
pub const DEAD: RecvOpcode = [b'D'];
