use crate::{protocol::data::pack_message, types::data::Message};

use std::net::UdpSocket;

pub type Address = String;

#[derive(Clone)]
pub struct BankService {
    address: Address,
}

impl BankService {
    pub fn new(address: Address) -> Self {
        println!("[INFO] Creo servicio de Banco address: '{:?}'", address);
        BankService { address }
    }
    pub fn send_message(
        &mut self,
        socket: &mut UdpSocket,
        msg: &Message,
    ) -> std::io::Result<usize> {
        println!(
            "[INFO] Enviando mensaje '{:?}' para Bank Service!",
            msg.action
        );
        let mut buf: Vec<u8> = Vec::new();
        pack_message(msg, &mut buf);
        socket.send_to(&buf, &self.address)
    }
}

#[derive(Clone)]
pub struct GenericService {
    address: Address,
}

impl GenericService {
    pub fn new(address: Address) -> Self {
        HotelService { address }
    }
    pub fn send_message(
        &mut self,
        socket: &mut UdpSocket,
        msg: &Message,
    ) -> std::io::Result<usize> {
        println!("[INFO] Enviando mensaje para Hotel Service");
        let mut buf: Vec<u8> = Vec::new();
        pack_message(msg, &mut buf);
        socket.send_to(&buf, &self.address)
    }
}

pub type HotelService = GenericService;
pub type AirlineService = GenericService;
