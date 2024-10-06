extern crate websocket;
extern crate serde;

mod application_message;
mod port_provider;
mod connector;
use connector::Connector;

use application_message::ApplicationMessage;

fn main() {
    
    let socket_addr = crate::port_provider::get_application_port();
    let mut server = websocket::sync::Server::bind(socket_addr).expect("Failed to create server");

    println!("Listening on {}", socket_addr);

    let mut client = server.get_connection(print_discarded_connection);

    println!("Connected to client, listening to messages");

    loop {

        let msg = ApplicationMessage::from_owned_message(client.recv_message().unwrap()).unwrap();
        println!("Message list is {}", msg.list_name);

    }
}

fn print_discarded_connection(fail_reason: &str) {

    println!("{}", fail_reason);
}
