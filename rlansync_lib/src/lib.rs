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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct CompletedCallback {
    userdata: *mut c_void,
    callback: extern "C" fn(*mut c_void, *mut c_char),
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