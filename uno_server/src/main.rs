mod models;

use std::{io::Read, net::SocketAddr, net::{TcpListener, TcpStream}, thread};

use local_ip_address::local_ip;
use uno_lib::models::packet::Packet;

fn main() {

    // Start info
    println!("This is a UNO Server");
    println!("");
    println!("Starting server....");

    // Gets the local ip
    let my_local_ip = local_ip().unwrap();

    // Bind the local ip to a TCP Listener
    let listener = TcpListener::bind(SocketAddr::new(my_local_ip,6969)).expect("Server failed Starting");


    // Starts to listen for Clients
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {

                // Starts a new thread for the client
                thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(err) => {

            }
        }

    }
}

fn handle_connection(mut stream: TcpStream){

    loop {
        
        // Buffer to read bytes from client
        let mut buffer: [u8; 8192] = [0; 8192];


        // Reads bytes from client
        match stream.read(&mut buffer) {
            Ok(size) => {

                let packet: Packet = Packet::new(&buffer);

                // Match for incoming
                match packet.action {
                    uno_lib::models::packet::Action::CreateGame => todo!(),
                    uno_lib::models::packet::Action::JoinGame => todo!(),
                    uno_lib::models::packet::Action::SpectateGame => todo!(),
                    _ => {
                        let _ = stream.shutdown(std::net::Shutdown::Both);
                        break;
                    }
                }

            }
            Err(err) => {
                println!("Error: {}", err);
                let _ = stream.shutdown(std::net::Shutdown::Both);
                break;
            }
        }

    }

}
