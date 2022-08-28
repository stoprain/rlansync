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
pub mod syncer;
mod protos;
mod utils;

#[cfg(not(feature = "swift"))]
#[allow(warnings)]
pub extern "C" fn swift_callback(json: &str) {
    let entries: Vec<FileInfo> = serde_json::from_str(json).unwrap();
    println!("swift_callback json len > {}", entries.len());
}

#[cfg(feature = "swift")]
pub use ffi::swift_callback;
use server::Server;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FileInfo {
    pub path: String,
    pub source: String,
    pub digest: String,
    pub tag: String,
    pub modify: u64,
    pub operation: String,
}

#[swift_bridge::bridge]
mod ffi {
    extern "Rust" {
        type RustApp;

        #[swift_bridge(init)]
        fn new(path: &str) -> RustApp;
        fn setup(&mut self);
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
    pub fn new(path: &str) -> Self {
        RustApp {
            server: server::Server::new(path),
        }
    }

    pub fn setup(&mut self) {
        mdns::setup_mdns();
        self.server.run();
        println!("######## start sync ########")
    }

    pub fn pull(&mut self, path: &str) {
        mdns::query_mdns(path.to_string());
    }

    pub fn update(&mut self, path: &str, tag: &str) {
        self.server.update(path, tag)
    }
}