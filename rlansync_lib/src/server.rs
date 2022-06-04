// use crate::scanner;


use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::thread;

use crate::scanner;

//https://stackoverflow.com/questions/30677258/how-do-i-import-from-a-sibling-module
pub struct Server {

}

impl Server {

    pub fn new() -> Self {
        Server {

        }
    }

    pub fn run(&mut self, pathbuf: &str) {    
        let mut scanner = scanner::Scanner::new();
        scanner.scan(pathbuf);

        let counter = Arc::new(Mutex::new(scanner));

        setup_tcp_listener(counter);
    }
}

//https://doc.rust-lang.org/book/ch20-01-single-threaded.html
fn setup_tcp_listener(scan: std::sync::Arc<std::sync::Mutex<scanner::Scanner>>) {
    std::thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
        println!("listener {:?}", listener);
        for stream in listener.incoming() {
            let counter = Arc::clone(&scan);
            match stream {
                Err(e)=> {eprintln!("failed: {}", e)}
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream, counter).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            } 
        }
    });
}

fn handle_client(mut stream: TcpStream, counter: std::sync::Arc<std::sync::Mutex<scanner::Scanner>>)-> Result<(), Error> {
    println!("incoming connection from: {}", stream.peer_addr()?);
    let scanner = counter.lock().unwrap();
    println!("{:?}", scanner.entries_modified);
    let mut buf = [0;512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {return Ok(())}
        let tmp = format!("{}", String::from_utf8_lossy(&buf).trim());
        eprintln!("getting {}",tmp);
        stream.write(&buf[..bytes_read])?;
    }
}
