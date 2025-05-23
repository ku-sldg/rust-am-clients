// main.rs (rust-am-client)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use rust_am_lib::copland::Term::asp;
use rust_am_lib::copland::ASP::APPR;

use rust_am_lib::tcp::*;
use lib::clientArgs::*;

// Other packages required to perform specific ASP action.
use std::fs;
use std::collections::HashMap;
use serde_json::Value;

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

fn extend_asp_params_w_appraisal_args (app_args_map:HashMap<ASP_ID, HashMap<TARG_ID, ASP_ARGS>>, attestation_params:ASP_PARAMS) -> ASP_PARAMS {

    let attestation_aspid = attestation_params.ASP_ID;
    let attestation_args = attestation_params.ASP_ARGS; 
    let attestation_plc = attestation_params.ASP_PLC;
    let attestation_targid = attestation_params.ASP_TARG_ID; 

    let x = app_args_map.get(&attestation_aspid);
    let y : Option<serde_json::Value> = 
    match x {
        Some (m) => 
        {
            m.get(&attestation_targid).cloned()
        }
        _ => 
        {
            let v = serde_json::Value::Object(serde_json::Map::new());
            Some(v)
        }

    };

    let z = 
        match y {
            Some (v) => {v}
            _ => {serde_json::Value::Object(serde_json::Map::new())}
        };


    let app_args_object: &serde_json::Map<String, serde_json::Value>= z.as_object()
                                                .expect("app_args NOT a JSON Object in fn `extend_asp_params_w_appraisal_args` ");

    let attestation_asp_args_key = "attestation_asp_args".to_string();
    let blah: &mut serde_json::Map<String, serde_json::Value> = &mut app_args_object.clone();
    let _ = blah.entry(attestation_asp_args_key).or_insert(attestation_args); //.expect("failed blah.insert");

    let new_args_object = serde_json::Value::Object(blah.clone());


    let res : ASP_PARAMS = 
        ASP_PARAMS { ASP_ID: attestation_aspid, ASP_ARGS: new_args_object, ASP_PLC: attestation_plc, ASP_TARG_ID: attestation_targid };

    res

}


fn extend_w_appraisal_args_et (app_args_map:HashMap<ASP_ID, HashMap<TARG_ID, ASP_ARGS>>, et:EvidenceT) -> EvidenceT {

    
    match et {
        rust_am_lib::copland::EvidenceT::mt_evt => et,
        rust_am_lib::copland::EvidenceT::nonce_evt(_) => et,
        rust_am_lib::copland::EvidenceT::left_evt(et1) =>  
            {let inner = extend_w_appraisal_args_et(app_args_map, *et1);
                rust_am_lib::copland::EvidenceT::left_evt(Box::new(inner))},
        rust_am_lib::copland::EvidenceT::right_evt(et2) =>  
            {let inner = extend_w_appraisal_args_et(app_args_map, *et2);
                rust_am_lib::copland::EvidenceT::right_evt(Box::new(inner))},
        rust_am_lib::copland::EvidenceT::split_evt(et1, et2) =>  
            {let inner1 = extend_w_appraisal_args_et(app_args_map.clone(), *et1);
             let inner2 = extend_w_appraisal_args_et(app_args_map.clone(), *et2);
                rust_am_lib::copland::EvidenceT::split_evt(Box::new(inner1), Box::new(inner2))}, 

        rust_am_lib::copland::EvidenceT::asp_evt(p, params, et1) =>  
                {let inner1 = extend_w_appraisal_args_et(app_args_map.clone(), *et1);
                    rust_am_lib::copland::EvidenceT::asp_evt(p, extend_asp_params_w_appraisal_args(app_args_map, params), Box::new(inner1))}

    }
}
fn extend_w_appraisal_args (app_args_map:HashMap<ASP_ID, HashMap<TARG_ID, ASP_ARGS>>, e:Evidence) -> Evidence {

    let old_et = e.EVIDENCET;

    let new_et = extend_w_appraisal_args_et(app_args_map, old_et);

    Evidence {
        RAWEV: e.RAWEV,
        EVIDENCET: new_et 
    }
}

fn aspc_args_swap(params:ASP_PARAMS, args_map:HashMap<ASP_ID, HashMap<TARG_ID, Value>>, keep_orig:bool) -> ASP_PARAMS {

    let id : ASP_ID = params.ASP_ID.clone();
    let targid : TARG_ID = params.ASP_TARG_ID.clone();
    let new_args: Value = 
      match args_map.get(&id) {
        Some (targs_map) => {

            match targs_map.get(&targid) {
                Some (val) => {
                    val.clone()
                }

                None => {
                    if keep_orig 
                    {params.ASP_ARGS} 
                else 
                    {serde_json::json!(null)}

                }
            }
        }
        None => {
            if keep_orig 
                {params.ASP_ARGS} 
            else 
                {serde_json::json!(null)}
        }
        
      };
      
    ASP_PARAMS { 
        ASP_ARGS: new_args,
        ASP_ID: params.ASP_ID,
        ASP_PLC: params.ASP_PLC,
        ASP_TARG_ID: params.ASP_TARG_ID,
    }

}

fn term_swap_args(t:Term, args_map:HashMap<ASP_ID, HashMap<TARG_ID, Value>>, keep_orig:bool) -> Term {
    match t {

        Term::asp(ref a) => {
            match a {
                ASP::ASPC(params) => {Term::asp(ASP::ASPC(aspc_args_swap(params.clone(), args_map, keep_orig)))}
                _ => {t}
            }
        }

        Term::att(q,t1) => {
            let t1: Term = term_swap_args(*t1, args_map, keep_orig);
            Term::att(q, Box::new(t1)) 
        }

        Term::lseq(t1,t2) => 
            { 
                let t1: Term = term_swap_args(*t1, args_map.clone(), keep_orig);
                let t2: Term = term_swap_args(*t2, args_map.clone(), keep_orig);

                Term::lseq(Box::new(t1), Box::new(t2))
            }

        Term::bseq(sp, t1,t2) => 
        { 
            let t1: Term = term_swap_args(*t1, args_map.clone(), keep_orig);
            let t2: Term = term_swap_args(*t2, args_map.clone(), keep_orig);

            Term::bseq(sp, Box::new(t1), Box::new(t2))
        }

        Term::bpar(sp, t1,t2) => 
        { 
            let t1: Term = term_swap_args(*t1, args_map.clone(), keep_orig);
            let t2: Term = term_swap_args(*t2, args_map.clone(), keep_orig);

            Term::bpar(sp, Box::new(t1), Box::new(t2))
        }
    }
}


fn main() -> std::io::Result<()> {

    let args = get_am_client_args()?;

    let term_filepath : String = args.term_filepath.clone();
    println!("\nterm_filepath arg: {}", term_filepath);

    /*
    let appraisal_args_filepath : String = "/Users/adampetz/Documents/Spring_2025/rust-am-clients/testing/rodeo_requests/micro_appr_args.json".to_string();
    println!("\nappraisal_args_filepath arg: {}", appraisal_args_filepath);
    */

    let att_server_uuid_string : String = args.server_uuid.clone();
    println!("server_uuid arg: {}", att_server_uuid_string);

    let client_uuid_string : String = args.client_uuid.clone();
    println!("client_uuid arg: {}", client_uuid_string);

    let term_contents = fs::read_to_string(term_filepath).expect("Couldn't read Term JSON file");
    eprintln!("\nTerm contents:\n{term_contents}");

    let t : Term = serde_json::from_str(&term_contents)?;
    eprintln!("\nDecoded Term as:");
    eprintln!("{:?}", t); // :? notation since formatter uses #[derive(..., Debug)] trait

    /*
    let appraisal_args_contents = fs::read_to_string(appraisal_args_filepath).expect("Couldn't read Appraisal ASP_ARGS JSON file");
    eprintln!("\nAppraisal ASP_ARGS contents:\n{appraisal_args_contents}");

    let app_asp_args :  HashMap<ASP_ID, HashMap<TARG_ID, ASP_ARGS>> = serde_json::from_str(&appraisal_args_contents)?;
    eprintln!("\nDecoded Appraisal ASP_ARGS as:");
    eprintln!("{:?}", app_asp_args);
    */

    let my_evidence: Evidence = rust_am_lib::copland::EMPTY_EVIDENCE.clone();

    let my_req_plc: Plc = "TOP_PLC".to_string();
    let my_to_plc: Plc = "P0".to_string();

    let my_att_session = get_session_from_am_client_args(&args)?;

    let maybe_attestation_args_string = args.b_attestation_asp_args_filepath.clone();

   // let my_term: Term = t;  
   let my_term: Term = 
        match maybe_attestation_args_string {
            Some(attestation_args_filepath) => {

                let attestation_args_contents = fs::read_to_string(attestation_args_filepath).expect("Couldn't read Appraisal ASP_ARGS JSON file");
                eprintln!("\nAttestation ASP_ARGS contents:\n{attestation_args_contents}");

                let app_asp_args :  HashMap<ASP_ID, HashMap<TARG_ID, ASP_ARGS>> = serde_json::from_str(&attestation_args_contents)?;
                eprintln!("\nDecoded Attestation ASP_ARGS as:");
                eprintln!("{:?}", app_asp_args);

                term_swap_args(t, app_asp_args, true)
            }

            _ => {t}

        };


    

    let vreq : ProtocolRunRequest = 
        ProtocolRunRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "RUN".to_string(), 
            REQ_PLC: my_req_plc, 
            TO_PLC: my_to_plc,
            TERM: my_term,
            EVIDENCE: my_evidence,
            ATTESTATION_SESSION: my_att_session.clone()};

    let req_str = serde_json::to_string(&vreq)?;

    let resp_str = am_sendRec_string_all(att_server_uuid_string, client_uuid_string, req_str)?;
    eprintln!("Got a TCP Response String: \n");
    eprintln!("{resp_str}\n");

    let resp : ProtocolRunResponse = serde_json::from_str(&resp_str)?;
    println!("Decoded ProtocolRunResponse: \n");
    println!("{:?}\n", resp);


    //let app_bool = true;
    let app_server = "127.0.0.1:5000".to_string();

    let maybe_appraisal_args_string = args.d_appraisal_asp_args_filepath.clone();

    let a_resp : ProtocolRunResponse = 
    match maybe_appraisal_args_string {
        Some(appraisal_args_filepath) => {

            let appraisal_args_contents = fs::read_to_string(appraisal_args_filepath).expect("Couldn't read Appraisal ASP_ARGS JSON file");
            eprintln!("\nAppraisal ASP_ARGS contents:\n{appraisal_args_contents}");

            let app_asp_args :  HashMap<ASP_ID, HashMap<TARG_ID, ASP_ARGS>> = serde_json::from_str(&appraisal_args_contents)?;
            eprintln!("\nDecoded Appraisal ASP_ARGS as:");
            eprintln!("{:?}", app_asp_args);

            //let a_resp = 
            //if app_bool {
                let app_term: Term = asp(APPR);  
                let app_evidence: Evidence = extend_w_appraisal_args(app_asp_args, resp.PAYLOAD.clone());
                let app_req : ProtocolRunRequest = 
                    ProtocolRunRequest {
                    TYPE: "REQUEST".to_string(), 
                    ACTION: "RUN".to_string(), 
                    REQ_PLC: "TOP_PLC".to_string(), 
                    TO_PLC: "P0".to_string(),
                    TERM: app_term,
                    EVIDENCE: app_evidence,
                    ATTESTATION_SESSION: my_att_session.clone()};

                let app_req_str = serde_json::to_string(&app_req)?;

                let app_resp_str = am_sendRec_string_all(app_server, "".to_string(), app_req_str)?;
                eprintln!("Got a TCP Response String: \n");
                eprintln!("{resp_str}\n");

                let app_resp : ProtocolRunResponse = serde_json::from_str(&app_resp_str)?;
                println!("Decoded ProtocolRunResponse: \n");
                println!("{:?}\n", app_resp);

                app_resp

            }
    | _ => {resp}
        };




    let appsumm_req : AppraisalSummaryRequest = 
        AppraisalSummaryRequest {
            TYPE: "REQUEST".to_string(), 
            ACTION: "APPSUMM".to_string(), 
            ATTESTATION_SESSION: my_att_session.clone(),
            EVIDENCE: a_resp.PAYLOAD
        };

    let appsumm_req_str: String = serde_json::to_string(&appsumm_req)?;

    let appsumm_resp_str = am_sendRec_string_all(args.server_uuid.clone(), args.client_uuid.clone(), appsumm_req_str)?;
    println!("Got a TCP Response String: \n");
    println!("{appsumm_resp_str}\n");

    let appsumm_resp : AppraisalSummaryResponse = serde_json::from_str(&appsumm_resp_str)?;
    eprintln!("Decoded AppraisalSummaryResponse: \n");
    eprintln!("{:?}\n", appsumm_resp);

    print_appsumm(appsumm_resp.PAYLOAD, appsumm_resp.SUCCESS);

     Ok (())

}


