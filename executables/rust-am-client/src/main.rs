// TEMPLATE.txt
// General structure for ASP's written in rust

//use anyhow::{Context, Result};
use rust_am_lib::copland::*; //{Plc, Attestation_Session};
use rust_am_lib::copland::EvidenceT::*;
use rust_am_lib::copland::ASP::*;
use rust_am_lib::copland::Term::*;
use rust_am_lib::copland::FWD::*;
use rust_am_lib::copland::EvInSig::*;
use rust_am_lib::copland::EvOutSig::*;
//use rust_am_lib::copland::SP::ALL;

// Packages required to perform specific ASP action.
// e.g.
//use sha2::{Digest, Sha256};

//mod clientargs;
//mod lib::tcp;
use lib::tcp::*;
//use clientargs::*;
//use tcp::*;


/*
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use rust_am_lib::copland::*;
use rust_am_lib::copland::Evidence::*;
use rust_am_lib::copland::RawEv::*;

//mod clientargs;
//mod tcp;
//use clientargs::*;
//use tcp::*;
*/

use std::fs;
use std::collections::HashMap;
//use hex;


fn main() -> std::io::Result<()> {

    /*
    let clientArgs = get_client_args()?;
    let clientArgs2 = clientArgs.clone();
    let clientArgs3 = clientArgs.clone();
    */

    

    let term_filepath = "/Users/adampetz/Documents/Spring_2025/rust-am-clients/testing/cert_appr.json"; //get_term_filepath(clientArgs); // "cert.json"
    //let att_server_uuid_string = get_att_uuid(clientArgs2);    //"localhost:5000";
    //let app_server_uuid_string = get_app_uuid(clientArgs3);   //"localhost:5003";

    let term_contents = fs::read_to_string(term_filepath).expect("Couldn't read Term JSON file");
    println!("Term contents:\n{term_contents}");

    #[allow(non_snake_case)]
    let t : Term = serde_json::from_str(&term_contents)?;//decode_gen(&term_contents)?;
    println!("Decoded Term as: \n");
    println!("\t{:?}\n", t); // :? formatter uses #[derive(..., Debug)] trait


    /*
    pub struct ASP_PARAMS {
    pub ASP_ID: ASP_ID,
    pub ASP_ARGS: ASP_ARGS,
    pub ASP_PLC: Plc,
    pub ASP_TARG_ID: TARG_ID,
}


    */

    let my_nonce : String = /*hex::encode(*/"PASSED".to_string(); /*); */
    let rawev_vec = vec![]; //vec![my_nonce];
    let my_plcmap: HashMap<Plc, String> = 
        HashMap::from([("P0".to_string(), "localhost:5000".to_string()), 
                       ("P1".to_string(), "localhost:5001".to_string()), 
                       ("P2".to_string(), "localhost:5002".to_string())
                       ]);
    let my_pubmap: HashMap<Plc, String> = HashMap::from([]);

    let attest_evsig : EvSig = 
            EvSig { FWD: EXTEND,
                    EvInSig: ALL, 
                    EvOutSig: OutN("1".to_string()) };

    let appraise_evsig : EvSig = 
    EvSig { FWD: EXTEND,
            EvInSig: ALL, 
            EvOutSig: OutN("1".to_string()) };

    let certificate_evsig : EvSig = 
    EvSig { FWD: EXTEND,
            EvInSig: ALL, 
            EvOutSig: OutN("1".to_string()) };

    let magic_appr_evsig : EvSig = 
    EvSig { FWD: REPLACE,
            EvInSig: ALL, 
            EvOutSig: OutN("1".to_string()) };

    let my_glob_type_env : HashMap<ASP_ID, EvSig> = HashMap::from ([("attest".to_string(), attest_evsig), 
                                                                    ("appraise".to_string(), appraise_evsig), 
                                                                    ("certificate".to_string(), certificate_evsig), 
                                                                    ("magic_appr".to_string(), magic_appr_evsig)]);
    let my_glob_comps : HashMap<ASP_ID, ASP_ID> = HashMap::from([("attest".to_string(), "magic_appr".to_string()),
                                                                 ("appraise".to_string(), "magic_appr".to_string()),
                                                                 ("certificate".to_string(), "magic_appr".to_string())]);

    let my_glob_context : GlobalContext = 
            GlobalContext { ASP_Types: my_glob_type_env, 
                            ASP_Comps: my_glob_comps};

    let my_evidence : Evidence = 
            Evidence { RAWEV: RawEv::RawEv (rawev_vec),
                       EVIDENCET: mt_evt };

    let my_att_session: Attestation_Session = 
            Attestation_Session { Session_Plc: "P0".to_string(), 
                                  Plc_Mapping: my_plcmap, 
                                  PubKey_Mapping: my_pubmap, 
                                  Session_Context: my_glob_context };

    let cert_asp_params: ASP_PARAMS = 
                ASP_PARAMS { ASP_ID: "attest".to_string(),
                ASP_ARGS: serde_json::Value::Bool(true), 
                ASP_PLC: "P0".to_string(),
                ASP_TARG_ID: "cert_targ".to_string() };

    let my_term: Term = t; //asp (ASPC (cert_asp_params));//t; //asp (ASPC (cert_asp_params));//(ASPC (ALL, EXTD("1".to_string()), cert_asp_params));   //asp(SIG); //t; 
    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: "TOP_PLC".to_string(), 
            TERM: my_term,
            EVIDENCE: my_evidence, //RawEv(rawev_vec),
            ATTESTATION_SESSION: my_att_session};

    let att_server_uuid_string= "localhost:5000".to_string();

    let req_str = serde_json::to_string(&vreq)?;//encode_gen(&vreq)?;

    let stream = connect_tcp_stream(att_server_uuid_string)?;
    println!("\nTrying to send ProtocolRunRequest JSON string: \n");
    println!("{req_str}\n");
    //let mut resp_str = "".to_string();

    let resp_str = tcp_sendRec_str(req_str,&stream /* , &mut resp_str */ )?;
    println!("Got a TCP Response String: \n");
    println!("\t{resp_str}\n");

    let resp : ProtocolRunResponse = serde_json::from_str(&resp_str)?; //decode_gen (&resp_str)?;
    println!("Decoded ProtocolRunResponse as: \n");
    println!("\t{:?}\n", resp); // :? formatter uses #[derive(..., Debug)] trait


/*

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

    let app_req_str = serde_json::to_string(&appreq)?;//encode_gen(&appreq)?;

    let app_stream = connect_tcp_stream(app_server_uuid_string)?;
    println!("\nTrying to send ProtocolAppraiseRequest JSON string: \n");
    println!("\t{app_req_str}\n");

    let app_resp_str = tcp_sendRec_str(app_req_str,&app_stream)?;
    println!("Got a TCP Response String: \n");
    println!("\t{app_resp_str}\n");

    let app_resp : ProtocolAppraiseResponse = serde_json::from_str(&app_resp_str)?;//decode_gen (&app_resp_str)?;
    println!("Decoded ProtocolAppraiseResponse as: \n");
    println!("\t{:?}\n", app_resp); // :? formatter uses #[derive(..., Debug)] trait



    {
        Ok (())
    }

    */

    Ok (())
}



/*

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    // Code for specific for this ASP.
    // This example computes the HASH of the file named in an argument for the ASP.
    // May return an Err Result, which will be captured in main.
    let filename = args
        .get("filepath")
        .context("filename argument not provided to ASP, hashfile_id")?;

    let env_var_key = "AM_ROOT";
    let env_var_string = match std::env::var(env_var_key) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable AM_ROOT")
        }
    };

    let filename_string = (*filename).clone();
    let filename_full = format! {"{env_var_string}{filename_string}"};

    eprint!("Attempting to read from file: {}\n", filename_full);

    let bytes = std::fs::read(filename_full)?; // Vec<u8>

    let hash = Sha256::digest(&bytes);
    Ok(vec![hash.to_vec()])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()


fn main() {
    handle_body(body);
}

*/


