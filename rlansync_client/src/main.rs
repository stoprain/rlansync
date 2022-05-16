use std::env;
mod iohelper;

// use std::io;
// use std::fs::{self, DirEntry};
// use std::path::Path;
mod scanner;

// use std::any::Any;
// use std::sync::{Arc, Mutex};
// use std::time::Duration;
// use zeroconf::prelude::*;
// use zeroconf::{MdnsService, ServiceRegistration, ServiceType, TxtRecord};

// use std::any::Any;
// use std::sync::Arc;
// use std::time::Duration;
// use zeroconf::prelude::*;
// use zeroconf::{MdnsBrowser, ServiceDiscovery, ServiceType};

// use simple_mdns::ServiceDiscovery;
use std::net::SocketAddr;
use std::str::FromStr;
use std::{thread, time};
use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::collections::HashMap;


/*
    * init client with configuration (machineId)
    
    * loop

        * build update to date file list (tree) (from scanner)
            https://doc.rust-lang.org/nightly/std/fs/fn.read_dir.html#examples
            https://stackoverflow.com/questions/58076576/how-do-i-build-an-iterator-for-walking-a-file-tree-recursively
        
        * compare with local file list (update local cache) // TODO won't recalculate hash

        * build file vs hash map (tree)
            https://stackoverflow.com/questions/29296038/implementing-a-mutable-tree-structure

        * search machines in lan
            https://docs.rs/mdns/3.0.0/mdns/discover/index.html
            https://bluejekyll.github.io/blog/posts/multicasting-in-rust/
            https://docs.rs/zeroconf/latest/zeroconf/
            https://www.reddit.com/r/rust/comments/npjngv/simple_dns_and_simple_mdns/
            https://users.rust-lang.org/t/mdns-and-dns-sd-for-the-trust-dns-resolver-feedback-desired/16718
            https://docs.rs/mdns-sd/latest/mdns_sd/
            https://serverfault.com/questions/136133/bonjour-mdns-broadcast-across-subnets
            
        * sync tree
            //https://stackoverflow.com/questions/66922989/convert-a-struct-to-byte-array-and-back-in-rust
        
        * pull changes (puller)
    
    *endloop
*/

fn main() {
    // let id = Uuid::new_v4();
    // println!("uuid = {}", id)

    let args: Vec<String> = env::args().collect();
    // // println!("{:?}", args)
    let filename = &args[1];
    // println!("Syncing {}", filename);

    // iohelper::test(filename.to_string());
    let mut scanner = scanner::Scanner::new();
    scanner.scan(filename);


    // let mut service = MdnsService::new(ServiceType::new("http", "tcp").unwrap(), 8080);
    // let mut txt_record = TxtRecord::new();
    // let context: Arc<Mutex<Context>> = Arc::default();

    // txt_record.insert("foo", "bar").unwrap();

    // service.set_registered_callback(Box::new(on_service_registered));
    // service.set_context(Box::new(context));
    // service.set_txt_record(txt_record);

    // let event_loop = service.register().unwrap();

    // loop {
    //     // calling `poll()` will keep this service alive
    //     event_loop.poll(Duration::from_secs(0)).unwrap();
    // }


    // let mut browser = MdnsBrowser::new(ServiceType::new("http", "tcp").unwrap());

    // browser.set_service_discovered_callback(Box::new(on_service_discovered));

    // let event_loop = browser.browse_services().unwrap();

    // loop {
    //     // calling `poll()` will keep this browser alive
    //     event_loop.poll(Duration::from_secs(0)).unwrap();
    // }

    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Create a service info.
    let service_type = "_mdns-sd-my-test._udp.local.";
    let instance_name = "my_instance";
    let host_ipv4 = "192.168.1.12";
    let host_name = "192.168.1.12.local.";
    let port = 5200;
    let mut properties = HashMap::new();
    properties.insert("property_1".to_string(), "test".to_string());
    properties.insert("property_2".to_string(), "1234".to_string());

    let my_service = ServiceInfo::new(
        service_type,
        instance_name,
        host_name,
        host_ipv4,
        port,
        Some(properties),
    );

    // Register with the daemon, which publishs the service.
    mdns.register(my_service).expect("Failed to register our service");

    // let future = mdns();
    // block_on(future);
    loop {
        thread::sleep(time::Duration::from_millis(10));
    }
}

// #[derive(Default, Debug)]
// pub struct Context {
//     service_name: String,
// }

// fn on_service_registered(
//     result: zeroconf::Result<ServiceRegistration>,
//     context: Option<Arc<dyn Any>>,
// ) {
//     let service = result.unwrap();

//     println!("Service registered: {:?}", service);

//     let context = context
//         .as_ref()
//         .unwrap()
//         .downcast_ref::<Arc<Mutex<Context>>>()
//         .unwrap()
//         .clone();

//     context.lock().unwrap().service_name = service.name().clone();

//     println!("Context: {:?}", context);

//     // ...
// }

// fn on_service_discovered(
//     result: zeroconf::Result<ServiceDiscovery>,
//     _context: Option<Arc<dyn Any>>,
// ) {
//     println!("Service discovered: {:?}", result.unwrap());

//     // ...
// }