use std::{env, time::Duration};
use rlansync_lib::{self, RustApp};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    if cfg!(feature="swift") {
    } else {
        let mut app = RustApp::new(filename);
        app.setup();
    }
    loop {std::thread::sleep(Duration::from_secs(1));}
}