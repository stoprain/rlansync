#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod strings;
pub mod scanner;

use std::os::raw::c_void;

// build for iOS
// https://blog.mozilla.org/data/2022/01/31/this-week-in-glean-building-and-deploying-a-rust-library-on-ios/#fnref1
// https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
// https://www.nickwilcox.com/blog/recipe_swift_rust_callback/
//https://bignerdranch.com/blog/building-an-ios-app-in-rust-part-2-passing-primitive-data-between-rust-and-ios/

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};
use get_if_addrs::IfAddr::{V4, V6};
use get_if_addrs::Ifv6Addr;
// #[no_mangle]
// pub extern "C" fn notify(from: *const c_char) {
//     let c_str = unsafe { CStr::from_ptr(from) };
//     let default = match c_str.to_str() {
//         Err(_) => "",
//         Ok(string) => string,
//     };


// }

// use std::thread;
use std::time::Duration;
use notify::{Watcher, RecursiveMode, watcher};
use std::sync::mpsc::channel;

// use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceInfo, ServiceEvent};
use std::collections::HashMap;
use gethostname::gethostname;
use substring::Substring;

use std::net::{Shutdown,TcpListener, TcpStream};
use std::thread;
use std::io::{Read,Write,Error};
use std::ops::Deref;

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

// impl Drop for CompletedCallback {
//     fn drop(&mut self) {
//         panic!("CompletedCallback must have explicit succeeded or failed call")
//     }
// }

#[no_mangle]
pub extern "C" fn notify(from: *const c_char, obj: SwiftObject) {
// pub extern "C" fn notify(from: *const c_char, callback: SwiftObject) {
    // thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(3));
    //     callback.succeeded()
    // });

//     let mut ip = String::new();
//     for iface in get_if_addrs::get_if_addrs().unwrap() {
//         if iface.name == "en0" {
//             match iface.addr {
//                 V4(v) => {
//                     ip = v.ip.to_string().to_owned()
//                 },
//                 V6(v) => {},
//             }
//         }
//         // println!("{:#?}", iface);
//     }

//     // Create a daemon
//     let mdns = ServiceDaemon::new().expect("Failed to create daemon");


// /*
// dns-sd -B _services._dns-sd._udp
// dns-sd -B _rlan-sync._udp
// dns-sd -L "rains-macb" _rlan-sync._udp
// dns-sd -L "rains-ipho" _rlan-sync._udp

// sudo killall -HUP mDNSResponder;
// */

//     // Create a service info.
//     let service_type = "_rlan._tcp.local.";

//     //receiver
//     // let mdns1 = ServiceDaemon::new().expect("Failed to create daemon");
//     let receiver = mdns.browse(service_type).expect("Failed to browse");
//     // Receive the browse events in sync or async. Here is
//     // an example of using a thread. Users can call `receiver.recv_async().await`
//     // if running in async environment.
//     std::thread::spawn(move || {
//         while let Ok(event) = receiver.recv() {
//             match event {
//                 ServiceEvent::ServiceResolved(info) => {
//                     println!("Resolved a new service: {}, {:?}, {}", info.get_fullname(), info.get_addresses(), info.get_port());
//                 }
//                 other_event => {
//                     println!("Received other event: {:?}", &other_event);
//                 }
//             }
//         }
//     });

//     //publish


//     let ss = gethostname().into_string().unwrap().to_lowercase();
//     let instance_name = ss.substring(0, 10);
//     // let instance_name = "my_instance";

//     let s = ip.clone() + ".local.";
//     let host_ipv4 = ip.as_str();
//     let host_name = s.as_str();
//     let port = 8888;
//     let mut properties = HashMap::new();
//     properties.insert("property_1".to_string(), "test".to_string());
//     properties.insert("property_2".to_string(), "1234".to_string());

//     println!("from {:?}", from);
//     println!("instance_name {:?}", instance_name);
//     println!("host_name {:?}", host_name);

//     let my_service = ServiceInfo::new(
//         service_type,
//         &instance_name,
//         host_name,
//         host_ipv4,
//         port,
//         Some(properties),
//     ).unwrap();

//     // Register with the daemon, which publishes the service.
//     mdns.register(my_service).expect("Failed to register our service");

    // setup_mdns();

    setup_tcp_listener();

    let c_str = unsafe { CStr::from_ptr(from) };
    let default = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let mut scanner = scanner::Scanner::new();
    scanner.scan(default);

    println!("{:?}", scanner.entries);

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(default, RecursiveMode::Recursive).unwrap();
    println!("watch {:?}", default);

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

fn setup_mdns() {

}

//https://doc.rust-lang.org/book/ch20-01-single-threaded.html
fn setup_tcp_listener() {
    std::thread::spawn(move || {
        let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
        println!("listener {:?}", listener);
        for stream in listener.incoming() {
            match stream {
                Err(e)=> {eprintln!("failed: {}", e)}
                Ok(stream) => {
                    thread::spawn(move || {
                        handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                    });
                }
            } 
        }
    });
}

fn handle_client(mut stream: TcpStream)-> Result<(), Error> {
    println!("incoming connection from: {}", stream.peer_addr()?);
    let mut buf = [0;512];
    loop {
        let bytes_read = stream.read(&mut buf)?;
        if bytes_read == 0 {return Ok(())}
        let tmp = format!("{}", String::from_utf8_lossy(&buf).trim());
        eprintln!("getting {}",tmp);
        stream.write(&buf[..bytes_read])?;
    }
}