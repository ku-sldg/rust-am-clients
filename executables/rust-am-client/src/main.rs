// main.rs (rust-am-client)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;

use lib::tcp::*;
use lib::clientArgs::*;

// Other packages required to perform specific ASP action.
use std::fs;
use std::collections::HashMap;
//use hex;


fn main() -> std::io::Result<()> {

    let args = get_am_client_args()?;

    let term_filepath : String = args.term_filepath;
    println!("\nterm_filepath arg: {}", term_filepath);

    let att_server_uuid_string : String = args.server_uuid;
    println!("server_uuid arg: {}", att_server_uuid_string);

    let plcmap_filepath : String = args.plcmap_filepath;
    println!("plcmap_filepath arg: {}", plcmap_filepath);

    let glob_type_env_filepath : String = args.env_filepath;
    println!("env_filepath arg: {}", glob_type_env_filepath);

    let glob_comps_filepath : String = args.glob_comps_filepath;
    println!("glob_comps_filepath arg: {}", glob_comps_filepath);


    let term_contents = fs::read_to_string(term_filepath).expect("Couldn't read Term JSON file");
    eprintln!("\nTerm contents:\n{term_contents}");

    let t : Term = serde_json::from_str(&term_contents)?;
    println!("\nDecoded Term as:");
    println!("{:?}", t); // :? notation since formatter uses #[derive(..., Debug)] trait

    let plcmap_contents = fs::read_to_string(plcmap_filepath).expect("Couldn't read plcmap JSON file");

    let my_plcmap: HashMap<Plc, String> = serde_json::from_str(&plcmap_contents)?;
    eprintln!("\nDecoded plcmap as:");
    eprintln!("{:?}", my_plcmap);

    let glob_type_env_contents = fs::read_to_string(glob_type_env_filepath).expect("Couldn't read glob_type_env JSON file");

    let my_glob_type_env: HashMap<ASP_ID, EvSig> = serde_json::from_str(&glob_type_env_contents)?;
    eprintln!("\nDecoded glob_type_env as:");
    eprintln!("{:?}", my_glob_type_env);

    let glob_comps_contents = fs::read_to_string(glob_comps_filepath).expect("Couldn't read glob_comps JSON file");

    let my_glob_comps: HashMap<ASP_ID, ASP_ID> = serde_json::from_str(&glob_comps_contents)?;
    eprintln!("\nDecoded glob_comps as:");
    eprintln!("{:?}", my_glob_comps);

    let my_glob_context : GlobalContext = 
            GlobalContext { ASP_Types: my_glob_type_env, 
                            ASP_Comps: my_glob_comps};

    let my_pubmap: HashMap<Plc, String> = HashMap::from([]);

    let my_evidence : Evidence = rust_am_lib::copland::EMPTY_EVIDENCE.clone();

    let my_att_session: Attestation_Session = 
            Attestation_Session { Session_Plc: "P0".to_string(), 
                                  Plc_Mapping: my_plcmap, 
                                  PubKey_Mapping: my_pubmap, 
                                  Session_Context: my_glob_context };

    let my_term: Term = t;  
    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: "TOP_PLC".to_string(), 
            TERM: my_term,
            EVIDENCE: my_evidence,
            ATTESTATION_SESSION: my_att_session};

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

    Ok (())
}


