use std::net::{SocketAddr, UdpSocket};

use crate::{
    protocol::data::send_msg_to,
    types::{common::BoxResult, data::Message},
};

// ----------------------------------------------------------------------------

pub struct ServiceDirectory {
    airline: SocketAddr,
    bank: SocketAddr,
    hotel: SocketAddr,
}

impl ServiceDirectory {
    pub fn new(airline: SocketAddr, bank: SocketAddr, hotel: SocketAddr) -> Self {
        ServiceDirectory {
            airline,
            bank,
            hotel,
        }
    }

    pub fn broadcast(&self, socket: &UdpSocket, msg: Message) -> BoxResult<()> {
        send_msg_to(socket, &msg, &self.airline)?;
        send_msg_to(socket, &msg, &self.bank)?;
        send_msg_to(socket, &msg, &self.hotel)?;

        Ok(())
    }
}
