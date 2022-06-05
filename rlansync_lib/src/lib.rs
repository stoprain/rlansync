#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

pub mod strings;
pub mod server;
pub mod scanner;

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

// use local_ip_address::local_ip;
use mdns_sd::{ServiceDaemon, ServiceInfo, ServiceEvent};
use std::collections::HashMap;
use gethostname::gethostname;

// impl Drop for CompletedCallback {
//     fn drop(&mut self) {
//         panic!("CompletedCallback must have explicit succeeded or failed call")
//     }
// }

mod protos;
use protobuf::Message;
use std::time::{SystemTime, UNIX_EPOCH};

use protos::generated_with_pure::example::{GetRequest, FileInfoRequest, FileDataRequest};
use protobuf::well_known_types::any::Any;
use protobuf::MessageField;
use server::SwiftObject;

#[no_mangle]
pub extern "C" fn notify(from: *const c_char, obj: SwiftObject) {

    let mut out_msg = FileInfoRequest::new();
    out_msg.from = 12345;

    let mut outm = GetRequest::new();
    outm.details = MessageField::some(Any::pack(&out_msg).unwrap());

    let out_bytes: Vec<u8> = out_msg.write_to_bytes().unwrap();

    let in_msg = GetRequest::parse_from_bytes(&out_bytes).unwrap();
    if outm.details.is::<FileInfoRequest>() {
        let request = outm.details.unpack::<FileInfoRequest>().unwrap().unwrap();
        assert_eq!(request.from, 12345);
        println!("{:?}", request)
    }
    // println!("in_msg {:?}", in_msg);
    // let content = match in_msg.details {
    //     MessageField(test) => println!("FileInfoRequest {:?}", test),
    //     MessageField(FileDataRequest) => println!("FileDataRequest"),
    // };
    // assert_eq!(in_msg.from, 12345);
    // println!("content {:?}", content);

    let c_str = unsafe { CStr::from_ptr(from) };
    let default = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let mut server = server::Server::new();
    server.run(default, obj);
}

fn setup_mdns() {

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
}