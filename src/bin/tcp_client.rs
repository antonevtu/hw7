use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5555").expect("connection failed");

    let mut buf = vec![0u8; 128];

    let commands: [i32; 6] = [1,2,0,2,1,2];
    for command in commands {
        stream.write_all(&command.to_be_bytes()).expect("fail to request");
        let n = stream.read(&mut buf).expect("fail to get reply");
        let string = String::from_utf8_lossy(&buf[..n]);
        println!("reply: {:?}", string);
        thread::sleep(Duration::from_secs_f32(0.5));
    }
}