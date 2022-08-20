use chrono::prelude::*;
use chrono::Duration;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::time::Instant;

fn main() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");
            let msg = String::from("What is the current time?");
            // Start timer at the instant of sending request
            let start = Instant::now();
            stream.write(msg.as_bytes()).unwrap();
            println!("Awaiting reply from server...");
            let mut data = [0 as u8; 50]; // using 50 byte buffer
            match stream.read(&mut data) {
                Ok(size) => {
                    // Time elapsed until response was received from the server
                    let process_delay_latency = start.elapsed();
                    println!("Latency: {:?}", process_delay_latency);
                    let reply_from_server = from_utf8(&data[0..size]).unwrap();
                    let server_time = reply_from_server.parse::<DateTime<Local>>().unwrap();
                    println!("Server Time: {:?}", server_time); // Time sent by server
                    let client_time =
                        server_time + Duration::from_std(process_delay_latency / 2).unwrap();
                    println!("Client Time: {:?}", client_time); // Actual client time
                }
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}
