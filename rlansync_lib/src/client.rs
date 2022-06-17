
mod protos;
mod utils;

use std::net::TcpStream;
use protobuf::Message;
use protos::generated_with_pure::example::{GetRequest, FileInfoRequest, FileDataRequest, FileInfoResponse};
use protobuf::well_known_types::any::Any;
use protobuf::MessageField;

fn main() {
    let stream = TcpStream::connect("0.0.0.0:8888").unwrap();

    let mut out_msg = FileInfoRequest::new();
    out_msg.from = 0;

    let mut outm = GetRequest::new();
    outm.details = MessageField::some(Any::pack(&out_msg).unwrap());

    let out_bytes: Vec<u8> = outm.write_to_bytes().unwrap();

    utils::write_head_and_bytes(&stream, &out_bytes);

    let payload = utils::read_head_and_bytes(&stream).unwrap();
    let res = FileInfoResponse::parse_from_bytes(&payload).unwrap();

    if res.fileInfos.len() > 0 {
        let first = &res.fileInfos[0];

        let mut out_msg = FileDataRequest::new();
        out_msg.digest = first.digest.to_owned();

        let mut outm = GetRequest::new();
        outm.details = MessageField::some(Any::pack(&out_msg).unwrap());

        let out_bytes: Vec<u8> = outm.write_to_bytes().unwrap();

        println!("> FileDataRequest digest {:?}", out_msg.digest);
        utils::write_head_and_bytes(&stream, &out_bytes);
    }
}