#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

use std::os::raw::c_int;
use std::os::raw::c_void;

#[no_mangle]
pub extern "C" fn shipping_rust_addition(a: c_int, b: c_int) -> c_int {
    a + b
}

// build for iOS
// https://blog.mozilla.org/data/2022/01/31/this-week-in-glean-building-and-deploying-a-rust-library-on-ios/#fnref1
// https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
// https://www.nickwilcox.com/blog/recipe_swift_rust_callback/

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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CompletedCallback {
    pub userdata: *mut c_void,
    pub callback: extern "C" fn(*mut c_void, *mut c_char),
}

unsafe impl Send for CompletedCallback {}

impl CompletedCallback {
    pub fn succeeded(self, result: String) {
        (self.callback)(self.userdata, result.as_ptr() as *mut c_char);
        std::mem::forget(self)
    }
    pub fn failed(self, result: String) {
        (self.callback)(self.userdata, result.as_ptr() as *mut c_char);
        std::mem::forget(self)
    }
}

// impl Drop for CompletedCallback {
//     fn drop(&mut self) {
//         panic!("CompletedCallback must have explicit succeeded or failed call")
//     }
// }

#[no_mangle]
pub extern "C" fn notify(from: *const c_char, callback: CompletedCallback) {
    // thread::spawn(move || {
    //     thread::sleep(Duration::from_secs(3));
    //     callback.succeeded()
    // });

    let mut ip = String::new();
    for iface in get_if_addrs::get_if_addrs().unwrap() {
        if iface.name == "en0" {
            match iface.addr {
                V4(v) => {
                    ip = v.ip.to_string().to_owned()
                },
                V6(v) => {},
            }
        }
        // println!("{:#?}", iface);
    }

    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");


/*
dns-sd -B _services._dns-sd._udp
dns-sd -B _rlan-sync._udp
dns-sd -L "rains-macb" _rlan-sync._udp
dns-sd -L "rains-ipho" _rlan-sync._udp

sudo killall -HUP mDNSResponder;
*/

    // Create a service info.
    let service_type = "_rlan._udp.local.";

    //receiver
    // let mdns1 = ServiceDaemon::new().expect("Failed to create daemon");
    let receiver = mdns.browse(service_type).expect("Failed to browse");
    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(info) => {
                    println!("Resolved a new service: {}, {:?}, {}", info.get_fullname(), info.get_addresses(), info.get_port());
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    });

    //publish


    let ss = gethostname().into_string().unwrap().to_lowercase();
    let instance_name = ss.substring(0, 10);
    // let instance_name = "my_instance";

    let s = ip.clone() + ".local.";
    let host_ipv4 = ip.as_str();
    let host_name = s.as_str();
    let port = 5200;
    let mut properties = HashMap::new();
    properties.insert("property_1".to_string(), "test".to_string());
    properties.insert("property_2".to_string(), "1234".to_string());

    println!("from {:?}", from);
    println!("instance_name {:?}", instance_name);
    println!("host_name {:?}", host_name);

    let my_service = ServiceInfo::new(
        service_type,
        &instance_name,
        host_name,
        host_ipv4,
        port,
        Some(properties),
    ).unwrap();

    // Register with the daemon, which publishes the service.
    mdns.register(my_service).expect("Failed to register our service");

    let c_str = unsafe { CStr::from_ptr(from) };
    let default = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let (tx, rx) = channel();
    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    watcher.watch(default, RecursiveMode::Recursive).unwrap();
    println!("watch {:?}", default);
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
                        callback.succeeded(s);
                    }
                    _ => {

                    }
                }
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}