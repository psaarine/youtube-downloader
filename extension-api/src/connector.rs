use std::net::{TcpStream};
use websocket::sync::Client;
use websocket::server::{NoTlsAcceptor, WsServer};
use std::net::TcpListener;
use websocket::server::upgrade::WsUpgrade;
use websocket::server::upgrade::sync::Buffer;

pub trait Connector {

    fn get_connection(&mut self, on_discard: fn(&str)) -> Client<TcpStream>;
}

impl Connector for WsServer<NoTlsAcceptor, TcpListener> {

    fn get_connection(&mut self, on_discard: fn(&str)) -> Client<TcpStream> {

        let try_map_connection = |conn| {

        if let Err(ref _e) = conn {

            on_discard("Tried to create connection but failed");
            return None;
        }
        let resp: WsUpgrade<TcpStream, Option<Buffer>> = conn.unwrap();
        let client = resp.accept();

        if let Err(_e) = client {
            on_discard("Could not create a client");
            return None;
        }

        return Some(client.unwrap());
        };
        return self.find_map(try_map_connection).unwrap();
    }
}

