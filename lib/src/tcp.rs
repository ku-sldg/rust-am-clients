use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, WriteBytesExt};

pub fn connect_tcp_stream (s:String) -> std::io::Result<TcpStream> {
    println!("\n{}{}", "Trying to connect to server at:  ", s);
    let stream = TcpStream::connect(s);
    stream
}

#[allow(non_snake_case)]
pub fn am_sendRec_string (s:String, mut stream:&TcpStream) -> std::io::Result<String> {
    let sbytes = s.as_bytes();
    let sbytes_len: u32 = sbytes.len().try_into().unwrap();

    // Write a buffer of bytes representing the (u32) size of the string to be sent
    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(sbytes_len).unwrap();
    stream.write_all(&wtr)?;

    // Write the string as bytes
    stream.write_all(s.as_bytes())?;

    // This is a hack to read 4 bytes from the stream (peeling off the response buffer size)
    // TODO:  We should probably use/decode this value in the future if we keep this approach
    let mut x:[u8; 4] = [0u8;4];
    stream.read_exact(&mut x)?;

    // Read in response string from stream
    let mut str_in : String = String::new();
    stream.read_to_string(&mut str_in)?;

    // Clone and return response string
    let str_out : String = str_in.clone();
    Ok (str_out)
}