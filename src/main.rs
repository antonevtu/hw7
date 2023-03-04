use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

struct SmartSocket {
    state: State,
    power: f32,
}

enum State {
    On,
    Off,
}

fn main() {
    println!("hello world!");

    let mut socket = SmartSocket {
        state: State::Off,
        power: 100.0,
    };

    let listener = TcpListener::bind("127.0.0.1:5555").expect("bind failed");

    while let Some(stream) = listener.incoming().next() {
        if stream.is_err() {
            continue;
        }

        let stream = stream.unwrap();
        let peer = stream.peer_addr();
        println!("connected: {:?}", peer);
        process_stream(stream, &mut socket);
        println!("disconnected: {:?}", peer);
    }
}

fn process_stream(mut stream: TcpStream, socket: &mut SmartSocket) {
    let mut buf = [0u8; 4];
    loop {
        if stream.read_exact(&mut buf).is_err() {
            break;
        }

        let request = u32::from_be_bytes(buf);
        println!("request: {request}");

        let reply = match request {
            0 => socket.turn_off(),
            1 => socket.turn_on(),
            2 => socket.get_power(),
            _ => unknown_request(),
        };

        if stream.write_all(&reply.as_bytes()).is_err() {
            break;
        }
    }
}

impl SmartSocket {
    fn turn_off(&mut self) -> String {
        self.state = State::Off;
        String::from("socket turned off")
    }

    fn turn_on(&mut self) -> String {
        self.state = State::On;
        String::from("socket turned on")
    }

    fn get_power(&self) -> String {
        match &self.state {
            State::On => format!("Consumption power: {} W", self.power),
            State::Off => format!("Consumption power: {} W. Socket turned off", 0),
        }
    }
}

fn unknown_request() -> String {
    String::from("unknown command")
}
