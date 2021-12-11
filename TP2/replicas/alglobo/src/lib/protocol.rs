// Send messages
pub type SendMessage = [u8; 1];
pub const EMPTY_MESSAGE: SendMessage = [0];
pub const REGISTER: SendMessage = [b'R'];
pub const FINISHED: SendMessage = [b'F'];

// Receive messages
pub const ACCEPTED: u8 = b'A';
pub const REJECTED: u8 = b'R';
pub const EOB: u8 = b'E';
pub const PING: u8 = b'P';
// --- broadcast msgs
pub const NEW: u8 = b'N';
pub const DEAD: u8 = b'D';
