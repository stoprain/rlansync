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

#[cfg(not(feature = "swift"))]
#[allow(warnings)]
pub extern "C" fn swift_callback(json: &str) {
    println!("swift_callback json > {}", json);
}

#[cfg(feature = "swift")]
pub use ffi::swift_callback;
use server::Server;

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RustApp;

        #[swift_bridge(init)]
        fn new() -> RustApp;
        fn setup(&mut self, path: &str);
        fn pull(&mut self, path: &str);
        fn update(&mut self, path: &str, tag: &str);
    }

    #[cfg(feature = "swift")]
    extern "Swift" {
        fn swift_callback(json: &str);
    }
}

pub struct RustApp {
    pub server: Server,
}

impl RustApp {
    pub fn new() -> Self {
        RustApp {
            server: server::Server::new(),
        }
    }

    pub fn setup(&mut self, path: &str) {
        mdns::setup_mdns();
        self.server.run(path);
    }

    pub fn pull(&mut self, path: &str) {
        mdns::query_mdns(path.to_string());
    }

    pub fn update(&mut self, path: &str, tag: &str) {
        self.server.update(path, tag)
    }
}