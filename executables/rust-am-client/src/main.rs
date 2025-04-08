// main.rs (rust-am-client)

// Custom package imports
use rust_am_lib::copland::*;
use rust_am_lib::copland::EvidenceT::*;
use rust_am_lib::copland::FWD::*;
use rust_am_lib::copland::EvInSig::*;
use rust_am_lib::copland::EvOutSig::*;
//use rust_am_lib::copland::ASP::*;
//use rust_am_lib::copland::Term::*;
//use rust_am_lib::copland::SP::ALL;

use lib::tcp::*;
use lib::clientArgs::*;

// Other packages required to perform specific ASP action.
use std::fs;
use std::collections::HashMap;
//use hex;


fn main() -> std::io::Result<()> {

    let args = get_am_client_args()?;

    let term_filepath : String = args.term_filepath;
    eprintln!("\n\nterm_filepath arg: {}\n\n", term_filepath);

    let att_server_uuid_string : String = args.server_uuid;
    eprintln!("\n\nserver_uuid arg: {}\n\n", att_server_uuid_string);

    let term_contents = fs::read_to_string(term_filepath).expect("Couldn't read Term JSON file");
    eprintln!("Term contents:\n{term_contents}");

    #[allow(non_snake_case)]
    let t : Term = serde_json::from_str(&term_contents)?;
    println!("Decoded Term as: \n");
    println!("\t{:?}\n", t); // :? notation since formatter uses #[derive(..., Debug)] trait

    //let my_nonce : String = "nonce_val".to_string();  //hex::encode("PASSED".to_string());
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

    /*
    let cert_asp_params: ASP_PARAMS = 
                ASP_PARAMS { ASP_ID: "attest".to_string(),
                ASP_ARGS: serde_json::Value::Bool(true), 
                ASP_PLC: "P0".to_string(),
                ASP_TARG_ID: "cert_targ".to_string() };
    */

    let my_term: Term = t; 
    //asp (ASPC (cert_asp_params));//t; //asp (ASPC (cert_asp_params));//(ASPC (ALL, EXTD("1".to_string()), cert_asp_params));   //asp(SIG); //t; 
    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: "TOP_PLC".to_string(), 
            TERM: my_term,
            EVIDENCE: my_evidence, //RawEv(rawev_vec),
            ATTESTATION_SESSION: my_att_session};

    let req_str = serde_json::to_string(&vreq)?;//encode_gen(&vreq)?;

    let stream = connect_tcp_stream(att_server_uuid_string)?;
    println!("\nTrying to send ProtocolRunRequest JSON string: \n");
    println!("{req_str}\n");

    let resp_str = am_sendRec_string(req_str,&stream)?;
    println!("Got a TCP Response String: \n");
    println!("\t{resp_str}\n");

    let resp : ProtocolRunResponse = serde_json::from_str(&resp_str)?;
    println!("Decoded ProtocolRunResponse as: \n");
    println!("{:?}\n", resp); // :? notation since formatter uses #[derive(..., Debug)] trait

    Ok (())
}


