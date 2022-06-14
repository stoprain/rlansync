
mod protos;
mod utils;

use std::net::TcpStream;
use protobuf::Message;
use protos::generated_with_pure::example::{GetRequest, FileInfoRequest, FileDataRequest};
use protobuf::well_known_types::any::Any;
use protobuf::MessageField;

fn main() {
    let stream = TcpStream::connect("0.0.0.0:8888").unwrap();

    let mut out_msg = FileInfoRequest::new();
    out_msg.from = 12345;

    let mut outm = GetRequest::new();
    outm.details = MessageField::some(Any::pack(&out_msg).unwrap());

    let out_bytes: Vec<u8> = outm.write_to_bytes().unwrap();

    // let mut buf = [0;512];
    utils::write_head_and_bytes(&stream, &out_bytes);

    // let payload = utils::read_head_and_bytes(&stream).unwrap();
    // println!("{:?}", payload)
}