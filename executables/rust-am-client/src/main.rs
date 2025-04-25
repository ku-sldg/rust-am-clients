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
use tokio::runtime::Runtime;
//use hex;

fn get_session_from_am_client_args (args:&AmClientArgs) -> std::io::Result<Attestation_Session> {


    let maybe_env_filepath = args.env_filepath.clone(); 
    let maybe_glob_comps_filepath = args.glob_comps_filepath.clone();

    let warning_message: String = 
        "NOTE:  One of env_filepath or glob_comps_filepath NOT provided as an arg.  Using the Attestation Session provided (or the DEFAULT if no session provided...)! ".to_string();

    match (maybe_env_filepath, maybe_glob_comps_filepath) {
        (Some (glob_type_env_filepath), Some (glob_comps_filepath)) => {
            println!("NOTE:  Overriding Attestation Session with provided env_filepath and glob_comps_filepath args");

            println!("env_filepath arg: {}", glob_type_env_filepath);
            println!("glob_comps_filepath arg: {}", glob_comps_filepath);
        
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

            let plcmap_filepath : String = args.plcmap_filepath.clone();
            println!("plcmap_filepath arg: {}", plcmap_filepath);
        
            let plcmap_contents = fs::read_to_string(plcmap_filepath).expect("Couldn't read plcmap JSON file");
        
            let my_plcmap: HashMap<Plc, String> = serde_json::from_str(&plcmap_contents)?;
            eprintln!("\nDecoded plcmap as:");
            eprintln!("{:?}", my_plcmap);
        
            let my_pubmap: HashMap<Plc, String> = HashMap::from([]);
        
            let my_session_plc: Plc = "P0".to_string();

            let my_att_session: Attestation_Session = 
                    Attestation_Session { Session_Plc: my_session_plc, 
                                          Plc_Mapping: my_plcmap, 
                                          PubKey_Mapping: my_pubmap, 
                                          Session_Context: my_glob_context };

            Ok (my_att_session)
        }
        _ => {
            println!("{warning_message}");

            let session_filepath : String = args.attestation_session_filepath.clone();
            println!("\nsession_filepath arg: {}", session_filepath);

            let session_contents = fs::read_to_string(session_filepath).expect("Couldn't read Attestation Session JSON file");
            eprintln!("\nAttestation Session contents:\n{session_contents}");
        
            let session : Attestation_Session = serde_json::from_str(&session_contents)?;
            println!("\nDecoded Attestation Session as:");
            println!("{:?}", session); // :? notation since formatter uses #[derive(..., Debug)] trait
            Ok (session)
         }
    }




}

fn main() -> std::io::Result<()> {

    let args = get_am_client_args()?;

    let term_filepath : String = args.term_filepath.clone();
    println!("\nterm_filepath arg: {}", term_filepath);

    let att_server_uuid_string : String = args.server_uuid.clone();
    println!("server_uuid arg: {}", att_server_uuid_string);

    let client_uuid_string : String = args.client_uuid.clone();
    println!("client_uuid arg: {}", client_uuid_string);

    let term_contents = fs::read_to_string(term_filepath).expect("Couldn't read Term JSON file");
    eprintln!("\nTerm contents:\n{term_contents}");

    let t : Term = serde_json::from_str(&term_contents)?;
    println!("\nDecoded Term as:");
    println!("{:?}", t); // :? notation since formatter uses #[derive(..., Debug)] trait

    let my_evidence: Evidence = rust_am_lib::copland::EMPTY_EVIDENCE.clone();

    let my_req_plc: Plc = "TOP_PLC".to_string();

    let my_att_session = get_session_from_am_client_args(&args)?;

    let my_term: Term = t;  
    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: my_req_plc, 
            TERM: my_term,
            EVIDENCE: my_evidence,
            ATTESTATION_SESSION: my_att_session};

    let req_str = serde_json::to_string(&vreq)?;

    let val = async {

    let stream = connect_tcp_stream(att_server_uuid_string, client_uuid_string).await?;
    println!("\nTrying to send ProtocolRunRequest: \n");
    println!("{req_str}\n");

    let resp_str = am_sendRec_string(req_str,stream).await?;
    eprintln!("Got a TCP Response String: \n");
    eprintln!("{resp_str}\n");

    let resp : ProtocolRunResponse = serde_json::from_str(&resp_str)?;
    println!("Decoded ProtocolRunResponse: \n");
    println!("{:?}\n", resp);

    Ok::<(), std::io::Error> (())
    };

    let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(val) {
        Ok(x) => x,
        Err(_) => println!("Runtime failure in rust-am-client main.rs"),
    };

    Ok (())
}


