// use crate::scanner;


use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Error};
use std::thread;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

use crate::syncer::Syncer;
use crate::{utils, swift_callback};

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
    pub sync_counter: Arc<Mutex<Syncer>>,
    root: String
}

impl Server {

    pub fn new(path: &str) -> Self {
        let syncer = Syncer::new(path);
        let counter =  Arc::new(Mutex::new(syncer));
        Server {
            sync_counter: counter,
            root: path.to_owned()
        }
    }

    pub fn pull(&mut self, addr: &str) {
        println!("start pull");
        let syncer = self.sync_counter.lock().unwrap();
        
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

            let infos = &syncer.file_infos;
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

            let s = self.root.to_owned() + &first.path;

            if is_exist {
                println!("> already exist {:?}", s);
                continue;
            } else if exist_path.len() > 0 {
                let ss =  self.root.to_owned() + &exist_path;
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

    pub fn run(&mut self) {
        let mut syncer = self.sync_counter.lock().unwrap();
        let s = syncer.get_file_list();

        setup_tcp_listener(self.sync_counter.clone());

        let ss = self.root.to_owned();

        swift_callback(&s);

        let counter = self.sync_counter.clone();

        std::thread::spawn(move || {
            let (tx, rx) = channel();
            let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
            watcher.watch(ss.to_owned(), RecursiveMode::Recursive).unwrap();
            println!("watch {:?}", ss);
    
            loop {
                match rx.recv() {
                    Ok(event) => {
                        // println!("{:?}", event);
                        let mut syncer = counter.lock().unwrap();
                        match event {
                            notify::DebouncedEvent::Remove(pathbuf) => {
                                syncer.remove(pathbuf.into_os_string().into_string().unwrap())
                            }
                            notify::DebouncedEvent::Create(pathbuf) => {
                                syncer.add(pathbuf.into_os_string().into_string().unwrap())
                            }
                            notify::DebouncedEvent::Write(pathbuf) => {
                                syncer.update(pathbuf.into_os_string().into_string().unwrap())
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

    pub fn update(&mut self, path: &str, tag: &str) {
        let mut syncer = self.sync_counter.lock().unwrap();
        syncer.update_tag(path, tag);
    }
}

//https://doc.rust-lang.org/book/ch20-01-single-threaded.html
fn setup_tcp_listener(counter: Arc<Mutex<Syncer>>) {
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

fn handle_client(stream: TcpStream, counter: Arc<Mutex<Syncer>>)-> Result<(), Error> {
    
    println!("<< incoming connection from: {}", stream.peer_addr()?);

    loop {
        let syncer = counter.lock().unwrap();

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

            let infos = &syncer.file_infos;
            for (_, value) in infos.into_iter() {
                if value.modify as i64 > request.from {
                    let mut info = FileInfo::new();
                    info.path = value.path.to_owned();
                    info.status = Status::CREATE.into();
                    info.digest = value.digest.to_owned();
                    res.fileInfos.push(info);
                    res.from = value.modify as i64;
                }
            }

            let out_bytes: Vec<u8> = res.write_to_bytes().unwrap();
            write_head_and_bytes(&stream, &out_bytes).unwrap();
        } else if req.details.is::<FileDataRequest>() {
            let request = req.details.unpack::<FileDataRequest>().unwrap().unwrap();
            println!("< FileDataRequest digest {:?}", request.digest);

            let mut res = FileDataResponse::new();
            res.digest = request.digest.to_owned();
            
            let infos = &syncer.file_infos;
            for (_, value) in infos.into_iter() {
                if value.digest == request.digest {
                    // let s = scanner.root.push_str(value.path.to_string());
                    let s = syncer.root.to_owned() + &value.path;
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