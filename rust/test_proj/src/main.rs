use openssl::sign::{Signer, Verifier};
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;

use serde::{Deserialize, Serialize};

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


fn connect_tcp_stream (s:String) -> std::io::Result<TcpStream> {
    println!("\n{}{}", "Trying to connect to server at:  ", s);
    let stream = TcpStream::connect(s);
    stream
}

fn tcp_sendRec_str (s:String, mut stream:&TcpStream) -> std::io::Result<String> {
    let mut s_out : String = "".to_string();
    stream.write_all(s.as_bytes());
    stream.read_to_string(&mut s_out);
    Ok (s_out)
}

fn encode_ProtocolRunRequest (v:&ProtocolRunRequest) -> std::result::Result<String, serde_json::Error> {
    serde_json::to_string(v)
}

fn decode_ProtocolRunRequest (s:&String) -> std::result::Result<ProtocolRunResponse, serde_json::Error> {
    serde_json::from_str(s)
}


fn main() {


    let v : Term = asp (SIG);
    let rawev_vec = vec!["anonce".to_string()];
    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: "TOP_PLC".to_string(), 
            TERM: v, 
            RAWEV: RawEv(rawev_vec)};

    let server_uuid = "localhost:5001";

    let maybe_json_req = encode_ProtocolRunRequest(&vreq);

    match maybe_json_req {
        Err (e) => { println! ("{}{:?}", "Error encoding this ProtocolRunRequest to JSON String:  ", vreq) } // :? formatter uses #[derive(..., Debug)]
        Ok (s) =>
        {
            let maybeStream = connect_tcp_stream(server_uuid.to_string());
            match maybeStream {
                Err (e) => { println! ("{}{}","error connecting to TCP server at:  ", server_uuid) }
                Ok (stream) => {
                    println!("\nTrying to send ProtocolRunRequest JSON string: \n");
                    println!("\t{s}\n");
                    let maybeRespString = tcp_sendRec_str(s,&stream);
                    match maybeRespString {
                        Err(e) => { println!("Error getting TCP Response String") }
                        Ok(respString) => 
                        {
                            println!("Got a TCP Response String: \n");
                            println!("\t{respString}\n");
                            
                            let maybeResp: std::result::Result<ProtocolRunResponse, serde_json::Error> = decode_ProtocolRunRequest (&respString);

                            match maybeResp {
                                Err(e) => { println!("Error Decoding ProtocolRunResponse...\n"); }
                                Ok(resp) =>{
                                    println!("Decoded ProtocolRunResponse as: \n");
                                    println!("\t{:?}\n", resp); // :? formatter uses #[derive(..., Debug)] trait
                                }
                            }   
                        }                        
                    }                   
                }               
            }
        }
    }


}
