use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();
    let mut buffer = [0 as u8; 50];
    let mut buffersize;
    let mut input = [0 as u8; 50];
    let mut inputsize;

    loop {
        inputsize = std::io::stdin()
            .read(&mut input)
            .expect("Failed to read line");

        println!(
            "^^ Sent this message ^^
Awaiting reply........."
        );

        stream.write(&input[..inputsize]).unwrap();

        buffersize = stream.read(&mut buffer).unwrap();

        if input[..].starts_with(&buffer[..]) {
            println!(
                "Received the same message => {}",
                String::from_utf8_lossy(&buffer[..buffersize])
            );
        } else {
            println!(
                "Received different message => {}",
                String::from_utf8_lossy(&buffer[..buffersize])
            );
        }
    }
}
