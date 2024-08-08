

use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

use hex;

mod copland;
mod test_json;

use crate::copland::copland::*;
use crate::Term::*;
use crate::ASP::*;
use crate::SP::*;
use crate::SP::*;
use crate::FWD::*;
//use crate::copland::copland::ASP_PARAMS;
use crate::RawEv::RawEv;


fn connect_tcp_stream (s:String) -> std::io::Result<TcpStream> {
    println!("\n{}{}", "Trying to connect to server at:  ", s);
    let stream = TcpStream::connect(s);
    stream
}

#[allow(non_snake_case)]
fn tcp_sendRec_str (s:String, mut stream:&TcpStream, s_out: & mut String) -> std::io::Result<()> {
    // let mut s_out : String = "hi".to_string(); // = "".to_string();
    stream.write_all(s.as_bytes())?;
    stream.read_to_string(s_out)?;
    Ok (())
}

#[allow(non_snake_case)]
fn encode_ProtocolRunRequest (v:&ProtocolRunRequest) -> std::result::Result<String, serde_json::Error> {
    serde_json::to_string(v)
}

#[allow(non_snake_case)]
fn decode_ProtocolRunRequest (s:&String) -> std::result::Result<ProtocolRunResponse, serde_json::Error> {
    serde_json::from_str(s)
}


fn main() {

    let v3 : Term = asp(ASPC(ALL, EXTD("1".to_string()), ASP_PARAMS{ ASP_ID:"hashfile_id".to_string(), ASP_ARGS:(HashMap::from([])), ASP_PLC:"P1".to_string(), ASP_TARG_ID:"hashfile_targ".to_string()}));
    let v1 : Term = asp (SIG);
    let v2 : Term = asp (SIG);
    //let v4 : Term = lseq (Box::new(v3), Box::new(v2));  //lseq (Box v1, Box v2);

    let v = v3;
    let my_nonce : String = hex::encode("anonce");
    let rawev_vec = vec![my_nonce];
    let my_plcmap: HashMap<Plc, String> = HashMap::from([("P1".to_string(), "localhost:5001".to_string())]);
    let my_pubmap: HashMap<Plc, String> = HashMap::from([("P1".to_string(), "".to_string())]);
    let my_att_session: Attestation_Session = Attestation_Session { Session_Plc: "P1".to_string(), Plc_Mapping: my_plcmap, PubKey_Mapping: my_pubmap };
    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: "TOP_PLC".to_string(), 
            TERM: v, 
            RAWEV: RawEv(rawev_vec),
            ATTESTATION_SESSION: my_att_session};

    let server_uuid = "localhost:5001";

    



    let maybe_json_req = encode_ProtocolRunRequest(&vreq);

    match maybe_json_req {
        Err (e) => { println! ("{}{:?}", "Error encoding this ProtocolRunRequest to JSON String:  ", vreq) } // :? formatter uses #[derive(..., Debug)]
        Ok (s) =>
        {
            #[allow(non_snake_case)]
            let maybeStream = connect_tcp_stream(server_uuid.to_string());
            match maybeStream {
                Err (e) => { println! ("{}{}","error connecting to TCP server at:  ", server_uuid) }
                Ok (stream) => {
                    println!("\nTrying to send ProtocolRunRequest JSON string: \n");
                    println!("\t{s}\n");
                    let mut respString = "".to_string();

                    #[allow(non_snake_case)]
                    let maybeRespRes = tcp_sendRec_str(s,&stream, &mut respString);
                    match maybeRespRes {
                        Err(e) => { println!("Error getting TCP Response String") }
                        Ok(_u) => 
                        {
                            println!("Got a TCP Response String: \n");
                            println!("\t{respString}\n");
                            
                            #[allow(non_snake_case)]
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
