

use std::fs;
use std::io::prelude::*;
use std::net::TcpStream;
use std::collections::HashMap;

use hex;

use serde::*;

mod copland;
mod test_json;

use crate::copland::copland::*;
use crate::Term::*;
use crate::ASP::*;
use crate::SP::*;
use crate::FWD::*;
use crate::RawEv::RawEv;
use crate::Evidence::*;
use crate::ProtocolRunResponse;


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

// std::result::Result<String, serde_json::Error>
fn encode_gen<T>(v: &T) -> std::io::Result<String>
    where T: ?Sized + Serialize 
    {
        #[allow(non_snake_case)]
        let maybeString = serde_json::to_string(v);
        match maybeString {
            Err (e) => {panic!("Error Encoding val: {e:?}")}
            Ok (s) => Ok (s)
        }
    }

// std::result::Result<T, serde_json::Error> 
fn decode_gen<'a, T>(s: &'a str) -> std::io::Result<T>
    where T: de::Deserialize<'a>
    {
        #[allow(non_snake_case)]
        let maybeVal = serde_json::from_str(s);
        match maybeVal {
            Err (e) => {panic!("Error Decoding val: {e:?}")}
            Ok (v) => Ok (v)
        }  
    }


fn main() -> std::io::Result<()> {

    let _v3 : Term = asp(ASPC(ALL, EXTD("1".to_string()), ASP_PARAMS{ ASP_ID:"hashfile_id".to_string(), ASP_ARGS:(HashMap::from([])), ASP_PLC:"P1".to_string(), ASP_TARG_ID:"hashfile_targ".to_string()}));
    let _v1 : Term = asp (SIG);
    let _v2 : Term = asp (SIG);
    //let v4 : Term = lseq (Box::new(v3), Box::new(v2));  //lseq (Box v1, Box v2);

    let _filehash_filepath = "filehash.json";
    let _cert_filepath = "cert.json";
    let _bg_filepath = "bg.json";
    let _parmut_filepath = "parmut.json";

    let term_filepath = _cert_filepath;
    // _parmut_filepath;
    // _bg_filepath;
    // _cert_filepath;
    // _filehash_filepath;

    let term_contents = fs::read_to_string(term_filepath).expect("Couldn't read Term JSON file");
    println!("Term contents:\n{term_contents}");

    #[allow(non_snake_case)]
    let t : Term = decode_gen(&term_contents)?;
    println!("Decoded Term as: \n");
    println!("\t{:?}\n", t); // :? formatter uses #[derive(..., Debug)] trait

    let my_nonce : String = hex::encode("anonce");
    let rawev_vec = vec![my_nonce];
    let my_plcmap: HashMap<Plc, String> = 
        HashMap::from([("P0".to_string(), "localhost:5000".to_string()), 
                       ("P1".to_string(), "localhost:5001".to_string()), 
                       ("P2".to_string(), "localhost:5002".to_string())
                       ]);
    let my_pubmap: HashMap<Plc, String> = HashMap::from([("P1".to_string(), "".to_string())]);
    let my_att_session: Attestation_Session = 
            Attestation_Session { Session_Plc: "P0".to_string(), 
                                  Plc_Mapping: my_plcmap, 
                                  PubKey_Mapping: my_pubmap };
    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: "TOP_PLC".to_string(), 
            TERM: t.clone(),
            RAWEV: RawEv(rawev_vec),
            ATTESTATION_SESSION: my_att_session.clone()};

    let server_uuid = "localhost:5000";

    let app_server_uuid = "localhost::5003";

    let req_str = encode_gen(&vreq)?;

    let stream = connect_tcp_stream(server_uuid.to_string())?;
    println!("\nTrying to send ProtocolRunRequest JSON string: \n");
    println!("\t{req_str}\n");
    let mut resp_str = "".to_string();

    let _u = tcp_sendRec_str(req_str,&stream, &mut resp_str)?;
    println!("Got a TCP Response String: \n");
    println!("\t{resp_str}\n");

    let resp : ProtocolRunResponse = decode_gen (&resp_str)?;
    println!("Decoded ProtocolRunResponse as: \n");
    println!("\t{:?}\n", resp); // :? formatter uses #[derive(..., Debug)] trait

    // TODO:  check for SUCCESS = true here...
    let res = resp.PAYLOAD;
    let et = nn("0".to_string());
    let t = t.clone();
    let sess = my_att_session.clone();

    let appreq =
            ProtocolAppraiseRequest {
            TYPE: "REQUEST".to_string(),
            ACTION: "RUN".to_string(),
            ATTESTATION_SESSION: sess, 
            TERM: t,
            REQ_PLC: "P0".to_string(),
            EVIDENCE: et, 
            RAWEV: res 
        };

    let json_app_req_str = encode_gen(&appreq)?;
    //match maybe_json_app_req {
        // Err (e) => { println! ("{}{:?}", "Error encoding this ProtocolAppraiseRequest to JSON String:  ", appreq) }
        //Ok (s) =>
        {
            Ok (())
        }


    }
