use openssl::sign::{Signer, Verifier};
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;

use serde::{Deserialize, Serialize};
use serde_json::Result;

use bytestring::*;

use std::io::prelude::*;
use std::io::{ErrorKind};
use std::net::TcpStream;

mod copland;
mod test_json;

use crate::copland::copland::*;
use crate::Term::*;
use crate::ASP::*;
use crate::RawEv::RawEv;

/*
fn create_tcp_stream () {
    let mut stream = TcpStream::connect("localhost:5000");
    stream
}
*/


fn test_tcp (t:&ProtocolRunRequest, stream:&TcpStream) -> serde_json::Result<ProtocolRunResponse> {

    //let maybeStream: std::result::Result<TcpStream, std::io::Error> = TcpStream::connect("localhost:5000");
   // match maybeStream {
      //  Ok(stream) => 
      //  {
            let mut de = serde_json::Deserializer::from_reader(stream);
            let _ = serde_json::to_writer(stream, t);
            let u: std::result::Result<ProtocolRunResponse, serde_json::Error> = ProtocolRunResponse::deserialize(&mut de);
            match u {
            Ok(v) => {Ok (v)}
            Err(e) => {Err(e.into())}
            }
        }

fn test_tcp_send_str (s:String, mut stream:&TcpStream) -> Result<String> {

    //let maybeStream: std::result::Result<TcpStream, std::io::Error> = TcpStream::connect("localhost:5000");
    // match maybeStream {
        //  Ok(stream) => 
        //  {
            //let mut de = serde_json::Deserializer::from_reader(stream);
            //let _ = serde_json::to_writer(stream, t);

            stream.write_all(s.as_bytes());

            let mut s_out : String = "".to_string();
            stream.read_to_string(&mut s_out);

            Ok (s_out)


            /*
            let u: std::result::Result<ProtocolRunResponse, serde_json::Error> = ProtocolRunResponse::deserialize(&mut de);
            match u {
            Ok(v) => {Ok (v)}
            Err(e) => {Err(e.into())}
            }
        */
}


fn main() {

// Generate a keypair
let keypair = Rsa::generate(2048).unwrap();
let keypair = PKey::from_rsa(keypair).unwrap();

let data = b"hello, world!";
let data2 = b"hola, mundo!";

// Sign the data
let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
signer.update(data).unwrap();
signer.update(data2).unwrap();
let signature = signer.sign_to_vec().unwrap();

// Verify the data
let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
verifier.update(data).unwrap();
verifier.update(data2).unwrap();
assert!(verifier.verify(&signature).unwrap());

//println!("Hello, world!");
//test_json::func();

//let v : Term = asp (ENC ("P7".to_string()));
let v : Term = asp (SIG);
let rawev_vec = vec!["anonce".to_string()];
let vreq : ProtocolRunRequest = 
    ProtocolRunRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "RUN".to_string(), 
        REQ_PLC: "TOP_PLC".to_string(), 
        TERM: v, 
        RAWEV: RawEv(rawev_vec)};

//let jsonv = serde_json::to_string(&v);

let jsonv_req = serde_json::to_string(&vreq);


/*
fn send_req_string (s:String) {
    let mut stream = TcpStream::connect("localhost:5000");
    stream.write_all(s.to_string());
    stream.read
}
*/

match jsonv_req {
    Ok (s) => // println! ("{s}"),
        { //send_req_string (s); ()
            println!("\nTrying to send ProtocolRunRequest JSON: \n");
            println!("{s}\n");
            let maybeStream = TcpStream::connect("localhost:5001");
            match maybeStream {
                Ok (stream) => {
                    let maybeRespString = test_tcp_send_str(s,&stream);        //test_tcp(&vreq, &stream);
                    match maybeRespString {
                        Ok(respString) => 
                        {
                            println!("Got a TCP Response String: \n");
                            println!("{respString}\n");
                            
                            
                            let maybeResp: std::result::Result<ProtocolRunResponse, serde_json::Error> = serde_json::from_str(&respString);

                            match maybeResp {
                                Ok(resp) =>{
                                    println!("Decoded ProtocolRunResponse: \n");
                                    println!("{:?}\n", resp);
                                }
                                Err(e) => {
                                    println!("Error Decoding ProtocolRunResponse...\n");
                                }


                            }   
                        }
                        Err(e) => 
                        {
                            println!("Error getting TCP Response String")
                        }
                    }
                    
                }
                Err (e) => {
                    println! ("error connecting to TCP stream")
                }
            }
        }
    Err (e) => {println! ("Error encoding to JSON String")}
}

/*
struct ProtocolRunRequest {
    TYPE:  String,
    ACTION:  String,
    REQ_PLC:  Plc,
    TERM:  Term, 
    RAWEV:  RawEv
}
*/




}
