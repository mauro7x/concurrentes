/* Shared protocol.rs between alglobo and directory */

pub type Opcode = [u8; 1];

// AlGlobo -> Directory
pub const REGISTER: Opcode = [b'R'];
pub const FINISHED: Opcode = [b'F'];

// Directory -> AlGlobo
pub const ACCEPTED: Opcode = [b'A'];
pub const REJECTED: Opcode = [b'R'];
pub const EOB: Opcode = [b'E'];
pub const NEW: Opcode = [b'N'];
pub const DEAD: Opcode = [b'D'];

// Not really dead code, used only by one
// of the services that share the file
#[allow(dead_code)]
pub const PING: Opcode = [b'P'];
