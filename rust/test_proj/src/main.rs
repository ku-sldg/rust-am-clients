mod copland;
mod clientargs;
mod tcp;
mod json;

//use pico_args::*;
use std::fs;
use std::collections::HashMap;

use hex;

use crate::clientargs::*;
use crate::tcp::*;
use crate::json::*;

use crate::copland::*;
use crate::RawEv::RawEv;
use crate::Evidence::*;
use crate::ProtocolRunResponse;


fn main() -> std::io::Result<()> {

    let clientArgs = get_client_args()?;
    let clientArgs2 = clientArgs.clone();
    let clientArgs3 = clientArgs.clone();

    let term_filepath = get_term_filepath(clientArgs); // "cert.json"
    let att_server_uuid_string = get_att_uuid(clientArgs2);    //"localhost:5000";
    let app_server_uuid_string = get_app_uuid(clientArgs3);   //"localhost:5003";

    let term_contents = fs::read_to_string(term_filepath).expect("Couldn't read Term JSON file");
    println!("Term contents:\n{term_contents}");

    #[allow(non_snake_case)]
    let t : Term = decode_gen(&term_contents)?;
    println!("Decoded Term as: \n");
    println!("\t{:?}\n", t); // :? formatter uses #[derive(..., Debug)] trait

    let my_nonce : String = hex::encode("PASSED");
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

    let req_str = encode_gen(&vreq)?;

    let stream = connect_tcp_stream(att_server_uuid_string)?;
    println!("\nTrying to send ProtocolRunRequest JSON string: \n");
    println!("\t{req_str}\n");
    //let mut resp_str = "".to_string();

    let resp_str = tcp_sendRec_str(req_str,&stream /* , &mut resp_str */ )?;
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
            ACTION: "APPRAISE".to_string(),
            ATTESTATION_SESSION: sess, 
            TERM: t,
            REQ_PLC: "P0".to_string(),
            EVIDENCE: et, 
            RAWEV: res 
        };

    let app_req_str = encode_gen(&appreq)?;

    let app_stream = connect_tcp_stream(app_server_uuid_string)?;
    println!("\nTrying to send ProtocolAppraiseRequest JSON string: \n");
    println!("\t{app_req_str}\n");

    let app_resp_str = tcp_sendRec_str(app_req_str,&app_stream)?;
    println!("Got a TCP Response String: \n");
    println!("\t{app_resp_str}\n");

    let app_resp : ProtocolAppraiseResponse = decode_gen (&app_resp_str)?;
    println!("Decoded ProtocolAppraiseResponse as: \n");
    println!("\t{:?}\n", app_resp); // :? formatter uses #[derive(..., Debug)] trait

    {
        Ok (())
    }


}
