use std::{io::prelude::*, time::Instant};
use std::net::TcpStream;

use nix::unistd::{fork, ForkResult};

fn main(){
    const CLIENT_NUMBER:i32 = 750;

    'main_loop: for _ in 0..CLIENT_NUMBER {
        while let Ok(ForkResult::Parent{ child: _, .. }) = fork() {
            client();
            break 'main_loop;
        }
    }
}

fn client() {
    let group_size: i16;
    let mut simulation_running;
    let id: i32;
    let mut response_time: u128 = 0;


    match TcpStream::connect("127.0.0.1:8080"){
        Ok(mut stream) => {

            //Init phase
            simulation_running = true;
            let mut buffer = [0 as u8; 10];
            
            while !stream.read(&mut buffer).is_ok(){};
            let val = bytes_to_data(&buffer);
            id = val.0;
            group_size = val.1.2;

            println!("Client {} connected,\nAwaiting for simulation to begin.", id);

            //Simulation phase
            while simulation_running {
                let instant = Instant::now();
                
                //Send coordinates
                buffer = data_to_bytes( &(id, (111,222,333)) );
                while !stream.write(&buffer).is_ok() {};

                //Receive coordinates
                for _ in 0..group_size {
                    while !stream.read(&mut buffer).is_ok() {};

                    let value = bytes_to_data(&buffer);
                    if true {
                        println!(
                            "Data received by {} =>ID:{} x:{} y:{} z:{}",
                            id,
                            value.0,
                            value.1.0,
                            value.1.1,
                            value.1.2
                        );
                    }
                }

                //Check if simulation ends (When a clients sends an ID equal to -1)
                while !stream.read(&mut buffer).is_ok() {};
                if bytes_to_data(&buffer).0 == -1 {
                    simulation_running = false;
                }

                response_time = (instant.elapsed().as_millis() + response_time) / 2;
            }
            
            //Closing phase
            buffer = time_to_bytes(response_time);
            while !stream.write(& buffer).is_ok() {};
            println!("Simulation ended for client: {}", id);

        },
        Err(_) => println!("Couldn't connect, closing...")
    };
}

fn time_to_bytes(value: u128) -> [u8; 10]{
    let value =  (value as u64).to_le_bytes();

    let mut buffer = [0 as u8; 10];
    buffer[..8].copy_from_slice(&value);

    return buffer;
}

fn bytes_to_data(buffer: &[u8; 10]) -> (i32,(i16,i16,i16)){
    let mut value=([0 as u8; 4],([0 as u8; 2],[0 as u8; 2],[0 as u8; 2]));
    value.0[..].copy_from_slice(&buffer[..4]);
    value.1.0[..].copy_from_slice(&buffer[4..6]);
    value.1.1[..].copy_from_slice(&buffer[6..8]);
    value.1.2[..].copy_from_slice(&buffer[8..]);
    
    (
        i32::from_le_bytes(value.0),
        (
            i16::from_le_bytes(value.1.0),
            i16::from_le_bytes(value.1.1),
            i16::from_le_bytes(value.1.2)
        )
    )
}

fn data_to_bytes(value: &(i32,(i16,i16,i16))) -> [u8; 10]{

    let mut buffer = [0 as u8; 10];

    buffer[..4].copy_from_slice(&value.0.to_le_bytes());
    buffer[4..6].copy_from_slice(&value.1.0.to_le_bytes());
    buffer[6..8].copy_from_slice(&value.1.1.to_le_bytes());
    buffer[8..].copy_from_slice(&value.1.2.to_le_bytes());

    buffer
}

/*
use std::{io::prelude::*, time::Duration};
use std::net::TcpStream;
use std::thread;

const GROUP_NUMBER: u32 = 20;
const GROUP_SIZE: usize = 20;
fn main() {
    let mut stream_group: Vec<TcpStream> = Vec::new();

    let mut connection_count = 0;

    let mut handles = vec![];
    for group_counter in 0..GROUP_NUMBER {
        for _ in 0..GROUP_SIZE {
            stream_group.push(TcpStream::connect("127.0.0.1:8080").unwrap());
            thread::sleep(Duration::from_millis(1)); 
        }
        let handle = thread::spawn(move || handle_group_client_connection(stream_group, group_counter));
        handles.push(handle);
        stream_group = Vec::new();
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn handle_group_client_connection(stream_group: Vec<TcpStream>, group_id:u32){
    let mut buffer = [0 as u8; 10];

    for stream in &stream_group{
        //stream.set_nonblocking(true).expect("Couldnt set nonblocking");
    }

    println!("Group {}.", group_id);
    loop {
        for mut stream in &stream_group{
            stream.set_nonblocking(true).expect("Couldnt set nonblocking");

            let value:(i16,i16,i16,u32)=(42,555,32, group_id);

            buffer = data_to_bytes(&value);

            stream.write(&buffer).expect("COULDNT WRITE");
        }

        for mut stream in &stream_group{
            stream.set_nonblocking(true).expect("Couldnt set nonblocking");
            match stream.read(&mut buffer){
                Ok(buffersize) => {
                    let value = bytes_to_data(&buffer);
                    println!("x:{} y:{} z:{} ID:{}", value.0, value.1, value.2, value.3);
                },
                Err(_error) => {}
            }

        }
        thread::sleep(Duration::from_millis(10));
    }
}
*/