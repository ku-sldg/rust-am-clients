use std::io::prelude::*;
use std::net::TcpStream;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};
//use socket::*;

pub fn connect_tcp_stream (s:String) -> std::io::Result<TcpStream> {
    println!("\n{}{}", "Trying to connect to server at:  ", s);
    let stream = TcpStream::connect(s);
    stream
}

#[allow(non_snake_case)]
pub fn tcp_sendRec_str (s:String, mut stream:&TcpStream /* , s_out: & mut String */ ) -> std::io::Result<String> {
    // let mut s_out : String = "hi".to_string(); // = "".to_string();
    let sbytes = s.as_bytes();
    let sbytes_len: u32 = sbytes.len().try_into().unwrap();

    let mut wtr = vec![];
    wtr.write_u32::<BigEndian>(sbytes_len).unwrap();
    //wtr.write_u32::<LittleEndian>(768).unwrap();
    //let sbytes_len_htonl = socket::htonl(sbytes_len);
    stream.write_all(&wtr)?;
    stream.write_all(s.as_bytes())?;
    let mut str_in : String = "".to_string();

    let mut x:[u8; 4] = [0u8;4];
    stream.read_exact(&mut x);

    stream.read_to_string(&mut str_in)?;
    let str_out : String = str_in.clone();
    Ok (str_out)
}

/*

fn sendRec_reqResp_gen<'a, T, R>(vreq: &T, server_uuid: String) -> std::io::Result<R>
    where T: ?Sized + Serialize,
          R: de::Deserialize<'a>,
          R: Debug
{
    let req_str = encode_gen(&vreq)?;

    let stream = connect_tcp_stream(server_uuid)?;
    println!("\nTrying to send Request JSON string: \n");
    println!("\t{req_str}\n");
    //let mut resp_str = "".to_string();

    let resp_str = tcp_sendRec_str(req_str,&stream)?;
    println!("Got a TCP Response String: \n");
    println!("\t{resp_str}\n");

    let resp_str_clone = resp_str.clone();

    //drop(resp_str);

    let resp : R = decode_gen (&resp_str)?;

    //println!("Decoded Response as: \n");
    // println!("\t{:?}\n", resp); // :? formatter uses #[derive(..., Debug)] trait
    Ok (resp)
}

*/