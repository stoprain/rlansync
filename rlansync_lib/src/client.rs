
// mod protos;
// mod utils;

// use std::net::TcpStream;
// use protobuf::Message;
// use protos::generated_with_pure::example::{GetRequest, FileInfoRequest, FileDataRequest, FileInfoResponse, FileDataResponse};
// use protobuf::well_known_types::any::Any;
// use protobuf::MessageField;
// use std::io::Write;
// use std::fs;

use std::env;
use rlansync_lib;
use rlansync_lib::server::SwiftObject;
use rlansync_lib::strings;
use std::os::raw::{c_void, c_char};

extern "C" fn ccallback(_: *mut c_void, _: strings::RustByteSlice) {

}

extern "C" fn destroy(_: *mut c_void) {

}

fn main() {

    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let c = SwiftObject {
        user: 0 as *mut c_void,
        destroy: destroy,
        callback_with_arg: ccallback,
    };

    let a = filename.as_ptr() as *const c_char;

    // let addr = "192.168.1.21:8888".to_string();
    // let b = addr.as_ptr() as *const c_char;
    // rlansync_lib::pull(a, b, c);
    rlansync_lib::pull(a, c);

    // let stream = TcpStream::connect("0.0.0.0:8888").unwrap();

    // let mut out_msg = FileInfoRequest::new();
    // out_msg.from = 0;

    // let mut outm = GetRequest::new();
    // outm.details = MessageField::some(Any::pack(&out_msg).unwrap());

    // let out_bytes: Vec<u8> = outm.write_to_bytes().unwrap();

    // utils::write_head_and_bytes(&stream, &out_bytes);

    // let payload = utils::read_head_and_bytes(&stream).unwrap();
    // let res = FileInfoResponse::parse_from_bytes(&payload).unwrap();

    // if res.fileInfos.len() > 0 {
    //     let first = &res.fileInfos[0];

    //     let mut out_msg = FileDataRequest::new();
    //     out_msg.digest = first.digest.to_owned();

    //     let mut outm = GetRequest::new();
    //     outm.details = MessageField::some(Any::pack(&out_msg).unwrap());

    //     let out_bytes: Vec<u8> = outm.write_to_bytes().unwrap();

    //     println!("> FileDataRequest digest {:?}", out_msg.digest);
    //     utils::write_head_and_bytes(&stream, &out_bytes);

    //     let payload = utils::read_head_and_bytes(&stream).unwrap();
    //     let res = FileDataResponse::parse_from_bytes(&payload).unwrap();

    //     fs::write("./result.txt", res.data);
}