use std::{io::prelude::*, time::{Duration, Instant}};
use std::net::TcpStream;
use rand::{thread_rng, Rng};

use std::thread;

const CONN_TIMEOUT: u64 = 1;

fn main(){
    const CLIENT_NUMBER:i32 = 60000;
	
	print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
	println!("Trying to connect all clients...");
	
	/*
    'main_loop: for _ in 0..CLIENT_NUMBER {
        while let Ok(ForkResult::Parent{ child: _, .. }) = fork() {
            client();
            break 'main_loop;
        }
    }*/

    let mut handles = vec![];

    for _ in 0..CLIENT_NUMBER {
        let handle = thread::spawn(||client());
        handles.push(handle);
    }
	
	println!("All clients connected.");
	println!("\n...\n");
	
    for handle in handles {
        handle.join().unwrap();
    }
	
	//println!("\nAll clients finished, closing.");
}

fn client() {
    let group_size: i16;
    let id: i32;
    let mut avg_response_time: Vec<u128> = vec![];
    let simulation_cycles: i16;

    loop{
        match TcpStream::connect("127.0.0.1:8080"){
            Ok(mut stream) => {
    
                //Init phase
                let mut buffer = [0 as u8; 10];
                stream.set_nodelay(true).unwrap();
                stream.set_read_timeout(Some(Duration::new(CONN_TIMEOUT, 0))).unwrap();
                //println!("Client connected through port: {},\nAwaiting for simulation to begin.", stream.local_addr().unwrap().port());
                
                while !stream.read(&mut buffer).is_ok(){};
                let val = bytes_to_data(&buffer);
                id = val.0;
                simulation_cycles = val.1.0;
                group_size = val.1.2;
                
    
                //Simulation phase
                for _ in 0..simulation_cycles {
                    let instant = Instant::now();
                    
                    //Send coordinates
                    buffer = data_to_bytes( &(id, (thread_rng().gen(),thread_rng().gen(),thread_rng().gen())) );
                    while !stream.write(&buffer).is_ok(){}
    
                    //Receive coordinates
                    for _ in 0..group_size {
                        while !stream.read(&mut buffer).is_ok(){}
                        let value = bytes_to_data(&buffer);
                        println!("Data received by {} =>ID:{} x:{} y:{} z:{}",id,value.0,value.1.0,value.1.1,value.1.2);
                    }
    
                    avg_response_time.push(instant.elapsed().as_millis());
                }
                
                //Closing phase
				let response_time = average(&avg_response_time);
                buffer = time_to_bytes(response_time);
                while !stream.write(& buffer).is_ok() {};
                //print!("\nSimulation ended for client: {}, response time: {}s", id, response_time as f64 / 1000.0);
                break;
            },
            Err(e) => {
                //println!("Couldn't connect, closing...\n{}", e);
                std::process::exit(0);
            }
        };
    }
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

fn average(values: &Vec<u128>) -> u128 {
    values.iter().sum::<u128>() as u128 / values.len() as u128
}