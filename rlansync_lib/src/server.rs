// use crate::scanner;


use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Error};
use std::thread;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

use crate::scanner::Scanner;
use crate::{scanner, utils, swift_callback};

use crate::protos;
use crate::utils::write_head_and_bytes;
use protos::generated_with_pure::example::{FileInfoResponse, FileInfo, FileInfoRequest, GetRequest, FileDataRequest, FileDataResponse};
use protos::generated_with_pure::example::file_info::Status;
use protobuf::Message;
use protobuf::well_known_types::any::Any;
use protobuf::MessageField;
use std::fs::File;
use std::fs;
use std::path::Path;

#[cfg(test)]
mod tests;


//https://stackoverflow.com/questions/30677258/how-do-i-import-from-a-sibling-module
pub struct Server {
    pub scanner_counter: Arc<Mutex<Scanner>>
}

impl Server {

    pub fn new() -> Self {
        let scanner = scanner::Scanner::new();
        let counter =  Arc::new(Mutex::new(scanner));
        Server {
            scanner_counter: counter
        }
    }

    pub fn pull(&mut self, path: &str, addr: &str) {
        println!("start pull");
        let mut scanner = self.scanner_counter.lock().unwrap();
        scanner.scan(path);
        
        let stream = TcpStream::connect(addr);
        let stream = match stream {
            Ok(s) => s,
            Err(e) => {
                println!("{}", e);
                return
            },
        };

        let mut out_msg = FileInfoRequest::new();
        out_msg.from = 0;
    
        let mut outm = GetRequest::new();
        outm.details = MessageField::some(Any::pack(&out_msg).unwrap());
    
        let out_bytes: Vec<u8> = outm.write_to_bytes().unwrap();
        
        utils::write_head_and_bytes(&stream, &out_bytes).unwrap();

        let payload = utils::read_head_and_bytes(&stream).unwrap();
        let res = FileInfoResponse::parse_from_bytes(&payload).unwrap();

        println!("start pull request {:?}", res);
    
        for value in res.fileInfos.into_iter() {
            let first = value;

            let infos = &scanner.entries_info;
            let mut is_exist = false;
            let mut exist_path = "".to_owned();
            for (_, value) in infos.into_iter() {
                if value.digest == first.digest {
                    if value.path == first.path {
                        is_exist = true;
                        break;
                    } else {
                        exist_path = value.path.to_owned();
                    }
                }
            }

            let s = path.to_owned() + &first.path;

            if is_exist {
                println!("> already exist {:?}", s);
                continue;
            } else if exist_path.len() > 0 {
                let ss =  path.to_owned() + &exist_path;
                println!("> move {:?} > {:?}", ss, s);
                fs::copy(ss, s).unwrap();
                continue;
            }
    
            let mut out_msg = FileDataRequest::new();
            out_msg.digest = first.digest.to_owned();
    
            let mut outm = GetRequest::new();
            outm.details = MessageField::some(Any::pack(&out_msg).unwrap());
    
            let out_bytes: Vec<u8> = outm.write_to_bytes().unwrap();
    
            println!("> FileDataRequest digest {:?}", out_msg.digest);
            utils::write_head_and_bytes(&stream, &out_bytes).unwrap();
    
            let payload = utils::read_head_and_bytes(&stream).unwrap();
            let res = FileDataResponse::parse_from_bytes(&payload).unwrap();
    
            println!("write to path {:?}", s);

            let path = Path::new(&s);
            let dir = path.parent().unwrap();
            fs::create_dir_all(dir).unwrap();
            fs::write(s, res.data).unwrap();
        }
    }

    pub fn run(&mut self, path: &str) {
        let mut scanner = self.scanner_counter.lock().unwrap();
        scanner.scan(path);
        let s = scanner.get_file_list();

        setup_tcp_listener(self.scanner_counter.clone());

        // let (tx, rx) = channel();
        // let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
        // watcher.watch(self.root.to_owned(), RecursiveMode::Recursive).unwrap();
        // println!("watch {:?}", self.root);

        let ss = path.to_owned();

        swift_callback(&s);

        let counter = self.scanner_counter.clone();

        std::thread::spawn(move || {
            // TODO update file info

            // let (tx, rx) = channel();

            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
            watcher.watch(ss.to_owned(), RecursiveMode::Recursive).unwrap();
            println!("watch {:?}", ss);
    
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
                                // let s = pathbuf.into_os_string().into_string().unwrap();
                                let mut scanner = counter.lock().unwrap();
                                let s = scanner.get_file_list();
                                // (obj.callback_with_arg)(obj.user, strings::RustByteSlice::from(s.as_ref()));
                                // println!("{:?}", scanner);
                                // (obj.callback_with_arg)(obj.user, strings::RustByteSlice::from(s.as_ref()))
                                swift_callback(&s);
                            }
                            _ => {
        
                            }
                        }
                    },
                    Err(e) => println!("watch error: {:?}", e),
                }
            }
        });
    }

    pub fn update(&mut self, _path: &str, _tag: &str) {
        let scanner = self.scanner_counter.lock().unwrap();
        println!("scanner.entries.len() {}", scanner.entries.len())
    }
}

//https://doc.rust-lang.org/book/ch20-01-single-threaded.html
fn setup_tcp_listener(counter: Arc<Mutex<scanner::Scanner>>) {
    std::thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
        println!("listener {:?}", listener);
        for stream in listener.incoming() {
            match stream {
                Err(e)=> {eprintln!("failed: {}", e)}
                Ok(stream) => {
                    let counter = Arc::clone(&counter);
                    thread::spawn(move || {
                        handle_client(stream, counter).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            } 
        }
    });
}

fn handle_client(stream: TcpStream, counter: Arc<Mutex<scanner::Scanner>>)-> Result<(), Error> {
    
    println!("<< incoming connection from: {}", stream.peer_addr()?);

    loop {
        let scanner = counter.lock().unwrap();

        let payload = utils::read_head_and_bytes(&stream);
        let p = match payload {
            Ok(v) => v,
            Err(_) => break,
        };
        let req = GetRequest::parse_from_bytes(&p).unwrap();
        println!("< GetRequest {:?}", req);
        if req.details.is::<FileInfoRequest>() {
            let request = req.details.unpack::<FileInfoRequest>().unwrap().unwrap();
            println!("< FileInfoRequest from {:?}", request.from);

            let mut res = FileInfoResponse::new();
            res.from = request.from;
            res.fileInfos = Vec::new();

            let infos = &scanner.entries_info;
            for (_, value) in infos.into_iter() {
                if value.modified as i64 > request.from {
                    let mut info = FileInfo::new();
                    info.path = value.path.to_owned();
                    info.status = Status::CREATE.into();
                    info.digest = value.digest.to_owned();
                    res.fileInfos.push(info);
                    res.from = value.modified as i64;
                }
            }

            let out_bytes: Vec<u8> = res.write_to_bytes().unwrap();
            write_head_and_bytes(&stream, &out_bytes).unwrap();
        } else if req.details.is::<FileDataRequest>() {
            let request = req.details.unpack::<FileDataRequest>().unwrap().unwrap();
            println!("< FileDataRequest digest {:?}", request.digest);

            let mut res = FileDataResponse::new();
            res.digest = request.digest.to_owned();
            
            let infos = &scanner.entries_info;
            for (_, value) in infos.into_iter() {
                if value.digest == request.digest {
                    // let s = scanner.root.push_str(value.path.to_string());
                    let s = scanner.root.to_owned() + &value.path;
                    let mut f = File::open(s).expect("no file found");
                    let metadata = File::metadata(&f).expect("unable to read metadata");
                    let mut buffer = vec![0; metadata.len() as usize];
                    f.read(&mut buffer).expect("buffer overflow");
                    res.data = buffer;
                    break
                }
            }

            let out_bytes: Vec<u8> = res.write_to_bytes().unwrap();
            write_head_and_bytes(&stream, &out_bytes).unwrap();
        }
    }

    println!(">> disconnect from: {}", stream.peer_addr()?);
    return Ok(());
}