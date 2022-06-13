use std::io;
use std::io::{Read, Write};
use std::net::TcpStream;

fn main() {
    let stream = TcpStream::connect("0.0.0.0:8888").unwrap();
    let mut buf = [0;512];
    write_head_and_bytes(&stream, &buf);
}

pub fn write_head_and_bytes(mut stream: &TcpStream, data: &[u8]) -> io::Result<()> {
    let buffer = (data.len() as u32).to_be_bytes();
    stream.write_all(&buffer)?;
    stream.write_all(data)?;
    Ok(())
}

pub fn read_head_and_bytes(mut stream: &TcpStream) -> io::Result<Vec<u8>> {
    let mut buffer = [0u8; 4];
    stream.read_exact(&mut buffer[..])?;
    let size = u32::from_be_bytes(buffer);
    let mut payload = vec![0; size as usize];
    stream.read_exact(&mut payload[..])?;
    Ok(payload)
}