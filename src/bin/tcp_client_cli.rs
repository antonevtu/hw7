use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::thread;
use std::time::Duration;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:5555").expect("connection failed");

    println!(
        "Select option: 
    0) Turn off; 
    1) Turn on; 
    2) Get state;
    3) Get power;
    other - exit"
    );

    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).unwrap();

        let selected = buf.trim();
        if (selected == "0") || (selected == "1") || (selected == "2") || (selected == "3"){
            let command: i32 = selected.parse().unwrap();

            println!("command: {:?}", command);

            stream
                .write_all(&command.to_be_bytes())
                .expect("fail to request");

            let mut read_buf = vec![0u8; 128];
            let n = stream.read(&mut read_buf).expect("fail to get reply");
            let string = String::from_utf8_lossy(&read_buf[..n]);
            println!("reply: {:?}", string);
            thread::sleep(Duration::from_secs_f32(0.5));
        } else {
            println!("exiting...");
            break;
        }
    }
}
