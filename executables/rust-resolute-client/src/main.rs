// main.rs (rust-resolute-client)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use rust_am_lib::copland::EvidenceT::*;

use lib::tcp::*;
use lib::clientArgs::*;

// Other packages required to perform specific ASP action.
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde_json::{Value};
//use hex;

#[derive(Serialize, Deserialize, Debug)]
pub struct ResoluteClientRequest {
    pub ResClientReq_attest_id: ASP_ID,
    pub ResClientReq_attest_args: Value
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResoluteClientResponse {
    pub ResClientResult_success: bool,
    pub ResClientResult_error: String,
    pub ResClientResult_term: Term, 
    pub ResClientResult_evidence: Evidence
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResoluteEnvironment {
    pub ResClientEnv_term: Term,
    pub ResClientEnv_session: Attestation_Session
}

pub type ResoluteEnvironmentMap = HashMap<ASP_ID, ResoluteEnvironment>;


fn resolute_to_am_request(res_req:ResoluteClientRequest, myPlc:Plc, init_evidence:Evidence, env:ResoluteEnvironmentMap) -> std::io::Result<ProtocolRunRequest> {

    let top_plc: Plc = myPlc;
    
    let asp_id_in: ASP_ID = res_req.ResClientReq_attest_id;
    let asp_args_in: ASP_ARGS = res_req.ResClientReq_attest_args;

    let my_env= env.get(&asp_id_in).expect("Term not found in ResoluteEnvironmentMap");

    let my_term_noargs = my_env.ResClientEnv_term.clone();
    let my_term = term_add_args (my_term_noargs, asp_args_in);
    let my_session: Attestation_Session = my_env.ResClientEnv_session.clone();

    let my_evidence : Evidence = init_evidence;

    let vreq : ProtocolRunRequest = 
    ProtocolRunRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "RUN".to_string(), 
        REQ_PLC: top_plc,
        TERM: my_term,
        EVIDENCE: my_evidence,
        ATTESTATION_SESSION: my_session};

    Ok (vreq)
}

fn do_response_app_summary(resp:ProtocolRunResponse) -> bool {
    let resp_evidence: Evidence = resp.PAYLOAD;
    let resp_rawev_wrapped: RawEv = resp_evidence.RAWEV;
    let resp_rawev: Vec<String> = match resp_rawev_wrapped {
        RawEv::RawEv(rawevt) => rawevt
    };
    let v = resp_rawev.iter().all(|x| *x == "".to_string());
    v
}

fn main() -> std::io::Result<()> {

    let args = get_resolute_client_args()?;

    let res_req_filepath : String = args.req_filepath;
    println!("\nres_req_filepath arg: {}", res_req_filepath);

    let att_server_uuid_string : String = args.server_uuid;
    println!("server_uuid arg: {}", att_server_uuid_string);

    let res_env_filepath : String = args.env_filepath;
    println!("res_env_filepath arg: {}", res_env_filepath);

    let res_req_contents = fs::read_to_string(res_req_filepath).expect("Couldn't read ResoluteClientRequest JSON file");
    eprintln!("\nTerm contents:\n{res_req_contents}");

    let res_req : ResoluteClientRequest = serde_json::from_str(&res_req_contents)?;
    println!("\nDecoded ResoluteClientRequest as:");
    println!("{:?}", res_req); // :? notation since formatter uses #[derive(..., Debug)] trait

    let res_env_contents = fs::read_to_string(res_env_filepath).expect("Couldn't read res_env JSON file");

    let my_res_env: ResoluteEnvironmentMap = serde_json::from_str(&res_env_contents)?;
    eprintln!("\nDecoded res_env as:");
    eprintln!("{:?}", my_res_env);

    let myPlc: Plc = "TOP_PLC".to_string();
    let my_evidence: Evidence = rust_am_lib::copland::EMPTY_EVIDENCE.clone();

    let vreq : ProtocolRunRequest = resolute_to_am_request(res_req, myPlc, my_evidence, my_res_env)?;

    let req_str = serde_json::to_string(&vreq)?;

    let stream = connect_tcp_stream(att_server_uuid_string)?;
    println!("\nTrying to send ProtocolRunRequest: \n");
    println!("{req_str}\n");

    let resp_str = am_sendRec_string(req_str,&stream)?;
    eprintln!("Got a TCP Response String: \n");
    eprintln!("{resp_str}\n");

    let resp : ProtocolRunResponse = serde_json::from_str(&resp_str)?;
    println!("Decoded ProtocolRunResponse: \n");
    println!("{:?}\n", resp);

    let success_bool: bool = do_response_app_summary(resp.clone());

    let res_resp: ResoluteClientResponse = 
        ResoluteClientResponse {
            ResClientResult_success: success_bool,
            ResClientResult_error: "".to_string(),
            ResClientResult_term: vreq.TERM,
            ResClientResult_evidence: resp.PAYLOAD
        };

    println!("ReslluteClientResponse: \n");
    println!("{:?}\n", res_resp);

    Ok (())
}


