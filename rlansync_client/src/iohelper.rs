extern crate notify;

use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
// use hex_literal::hex;
// use sha2::{Sha256, Sha512, Digest};
use std::fs::File;
use std::io;
use std::io::{BufReader, Read, Write};
use ring::digest::{Context, Digest, SHA256};
use std::error::Error;
mod datastore;
use data_encoding::HEXUPPER;

pub fn test(path: String) {
    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(path, RecursiveMode::Recursive).unwrap();
    loop {
        match rx.recv() {
            Ok(event) => {
                println!("{:?}", event);
                match event {
                    notify::DebouncedEvent::Remove(pathbuf) => {
                        println!("Remove pathbuf {:?}", pathbuf);
                    }
                    notify::DebouncedEvent::Create(pathbuf) => {
                        // let mut file = File::open(pathbuf);
                        // let mut sha256 = Sha256::new();
                        // io::copy(&mut file, &mut sha256)?;
                        // let hash = sha256.result();
                        // println!("Create pathbuf {:?}", hash);
                        let path = pathbuf.into_os_string().into_string().unwrap();
                        let input = File::open(&path).unwrap();
                        let reader = BufReader::new(input);
                        let digest = sha256_digest(reader).unwrap();
                        println!("{:?}", digest);

                        let datastore = datastore::Datastore::new();
                        datastore.save(path, HEXUPPER.encode(digest.as_ref()));
                    }
                    _ => {

                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
    //https://docs.rs/notify/3.0.1/notify/enum.DebouncedEvent.html
    //Create, Write, Rename, Remove

}

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Box<dyn Error>> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}