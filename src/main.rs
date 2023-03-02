use std::io::{Read, Write};
use std::net::{TcpStream, TcpListener};

#[derive(Default)]
struct SmartSocket {
    state: bool,
    power: f32,
}

fn main() {
    println!("hello world!");

    let buf = b"get_power|||bar";

    let request = String::from_utf8_lossy(buf);
        let x = request.split("|||");

    let mut socket:SmartSocket = Default::default();

    let listener = TcpListener::bind("127.0.0.1:5555").expect("bind failed");






    while let Some(stream) = listener.incoming().next() {
        if stream.is_err() {
            continue;
        }

        let stream = stream.unwrap();
        let peer = stream.peer_addr();
        println!("connected: {:?}", peer);
        process_stream(stream, &socket);
        println!("disconnected: {:?}", peer);
    }
}

fn process_stream(mut stream: TcpStream, socket: &SmartSocket) {
    let mut buf = [0u8, 128];
    loop {
        if stream.read(&mut buf).is_err() {
            break;
        }

        let request = String::from_utf8_lossy(&buf);
        let x = request.split("|||");
    }
}