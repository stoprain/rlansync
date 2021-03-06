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
pub mod mdns;
pub mod database;
mod protos;
mod utils;

// build for iOS
// https://blog.mozilla.org/data/2022/01/31/this-week-in-glean-building-and-deploying-a-rust-library-on-ios/#fnref1
// https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-06-rust-on-ios.html
// https://www.nickwilcox.com/blog/recipe_swift_rust_callback/
//https://bignerdranch.com/blog/building-an-ios-app-in-rust-part-2-passing-primitive-data-between-rust-and-ios/

use std::os::raw::{c_char};
use std::ffi::{CStr};

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

// impl Drop for CompletedCallback {
//     fn drop(&mut self) {
//         panic!("CompletedCallback must have explicit succeeded or failed call")
//     }
// }

// use std::time::{SystemTime, UNIX_EPOCH};


use server::SwiftObject;

#[no_mangle]
pub extern "C" fn notify(from: *const c_char, obj: SwiftObject) {

    mdns::setup_mdns();

    let c_str = unsafe { CStr::from_ptr(from) };
    let default = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };

    let mut server = server::Server::new(default.to_string());
    server.run(obj);
}

#[no_mangle]
pub extern "C" fn pull(from: *const c_char, obj: SwiftObject) {
    mdns::query_mdns(from, &obj);
}