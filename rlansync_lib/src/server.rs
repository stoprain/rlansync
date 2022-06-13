// use crate::scanner;


use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write, Error};
use std::thread;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::os::raw::c_void;
// use substring::Substring;
use std::ops::Deref;

use crate::scanner;
use crate::strings;

#[cfg(test)]
mod tests;


//https://stackoverflow.com/questions/30677258/how-do-i-import-from-a-sibling-module
pub struct Server {

}

impl Server {

    pub fn new() -> Self {
        Server {

        }
    }

    pub fn run(&mut self, pathbuf: &str, obj: SwiftObject) {    
        let mut scanner = scanner::Scanner::new();
        scanner.scan(pathbuf);

        let counter = Arc::new(Mutex::new(scanner));

        setup_tcp_listener(counter);

        let (tx, rx) = channel();
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
        watcher.watch(pathbuf, RecursiveMode::Recursive).unwrap();
        println!("watch {:?}", pathbuf);
    
        // std::thread::spawn(move || {
    
        // });
    
        loop {
            match rx.recv() {
                Ok(event) => {
                    println!("{:?}", event);
                    match event {
                        notify::DebouncedEvent::Remove(pathbuf) => {
                            println!("Remove pathbuf {:?}", pathbuf);
                        }
                        notify::DebouncedEvent::Create(pathbuf) => {
                            println!("Create pathbuf {:?}", pathbuf);
                            let s = pathbuf.into_os_string().into_string().unwrap();
                            (obj.callback_with_arg)(obj.user, strings::RustByteSlice::from(s.as_ref()));
                        }
                        _ => {
    
                        }
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }
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
    let infos = &scanner.entries_info;
    for (key, value) in infos.into_iter() {
        println!("{} / {}", key, value);
    }
    // let mut buf = [0;512];
    // loop {
    //     let bytes_read = stream.read(&mut buf)?;
    //     if bytes_read == 0 {return Ok(())}
    //     let tmp = format!("{}", String::from_utf8_lossy(&buf).trim());
    //     eprintln!("getting {}",tmp);
    //     stream.write(&buf[..bytes_read])?;
    // }
    Ok(())
}

#[repr(C)]
pub struct SwiftObject {
    pub user: *mut c_void,
    pub destroy: extern fn(user: *mut c_void),
    pub callback_with_arg: extern fn(user: *mut c_void, arg: strings::RustByteSlice),
}

unsafe impl Send for SwiftObject {}

struct SwiftObjectWrapper(SwiftObject);

impl Deref for SwiftObjectWrapper {
    type Target = SwiftObject;

    fn deref(&self) -> &SwiftObject {
        &self.0
    }
}

impl Drop for SwiftObjectWrapper {
    fn drop(&mut self) {
        (self.destroy)(self.user);
    }
}