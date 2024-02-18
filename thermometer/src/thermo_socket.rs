use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use crate::thermometr::Thermometr;

pub struct ThermoSocket {
    socket_addr: SocketAddr,
    thermometer: Thermometr,
}

impl ThermoSocket {
    pub fn new(addr: [u8;4], port: u16) -> Self {
        let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::from(addr)), port);

        let thermo = Thermometr::new();    
        Self { socket_addr: socket, thermometer: thermo}
    }

    pub fn bind_socket(&self) -> Result<UdpSocket, std::io::Error> {
        let to_socket_addr = self.socket_addr;
        UdpSocket::bind(to_socket_addr)
    }

    pub fn get_addr(&self) -> String {
        self.socket_addr.to_string()
    }

    pub fn get_temp(&mut self) -> i32 {
        self.thermometer.random_temp()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_new_thermosocket() {
        let _new_thermo = ThermoSocket::new([127, 0, 0, 1], 54433);
    }

    #[test]
    fn test_bind_thermosocket() {
        let new_thermo = ThermoSocket::new([127, 0, 0, 1], 8080);
        new_thermo.bind_socket().expect("All is bad");
    }

    #[test]
    fn test_get_temp() {
        let mut new_thermo = ThermoSocket::new([127, 0, 0, 1], 8080);
        new_thermo.bind_socket().expect("All is bad");
        new_thermo.get_temp();
    }

    #[test]
    fn test_get_addr() {
        let new_thermo = ThermoSocket::new([127, 0, 0, 1], 8080);
        new_thermo.bind_socket().expect("All is bad");
        let addr_thermo = new_thermo.get_addr();
        assert_eq!(addr_thermo, new_thermo.socket_addr.to_string())
    }
}