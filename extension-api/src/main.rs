extern crate websocket;
use std::net::{TcpStream};
use std::io::Write;
use websocket::server::{InvalidConnection};
use websocket::server::upgrade::WsUpgrade;
use websocket::server::upgrade::sync::Buffer;
use websocket::client::sync::Client;
use websocket::ws::dataframe::DataFrame;
use core::net::Ipv4Addr;
use std::net::SocketAddr;
use std::net::IpAddr;

type ConnectionResponse = Result<WsUpgrade<TcpStream, Option<Buffer>>, InvalidConnection<TcpStream, Buffer>>;

fn main() {
    
    let addr = IpAddr::V4(Ipv4Addr::LOCALHOST);
    let port: u16 = 1234;
    let socket_addr = SocketAddr::new(addr, port);
    let server = websocket::sync::Server::bind(socket_addr);

    if let Err(ref e) = server {

        println!("Failed to create server:{}", e);
    } else {

        println!("Listening on {}", socket_addr);
    }

    let mut server = server.unwrap();
    let mut client = server.find_map(try_map_connection).unwrap();

    println!("Connected to client, listening to messages");

    loop {

        let dataframe = client.recv_dataframe().unwrap();
        let mut writer = std::io::BufWriter::new(std::io::stdout());
        let resp = dataframe.write_to(&mut writer, false);

        if !resp.is_ok() {

            println!("Failed to read dataframe");
        } else {

            writer.flush();
        }


    }
}

pub fn try_map_connection(resp: ConnectionResponse) -> Option<Client<TcpStream>>{

    if let Err(ref _e) = resp {

        println!("Tried to create connection but failed");
        return None;
    }


    let resp = resp.unwrap();
    let client = resp.accept();

    if let Err(e) = client {

        println!("Could not create client, {}", e.1);
        return None;
    } else {

        let client_container = Some(client.unwrap());
        return client_container;
    }
}
