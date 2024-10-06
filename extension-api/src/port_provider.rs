use core::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::IpAddr;

pub fn get_application_port() -> SocketAddr {

    let addr = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let port: u16 = 1234;
    return SocketAddr::new(addr, port);
}
