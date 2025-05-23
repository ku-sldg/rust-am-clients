// main.rs (rust-rodeo-client)

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
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RodeoClientRequest {
    pub RodeoClientReq_attest_id: String,
    pub RodeoClientReq_attest_args: HashMap<ASP_ID, HashMap<TARG_ID, Value>>,
    pub RodeoClientReq_appraise_args: HashMap<ASP_ID, HashMap<TARG_ID, Value>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RodeoClientResponse {
    pub RodeoClientResult_success: bool,
    pub RodeoClientResult_error: String,
    pub RodeoClientResult_term: Term, 
    pub RodeoClientResult_evidence: Evidence
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RodeoEnvironment {
    pub RodeoClientEnv_term: Term,
    pub RodeoClientEnv_session: Attestation_Session
}

pub type RodeoEnvironmentMap = HashMap<ASP_ID, RodeoEnvironment>;

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


fn rodeo_to_am_request(res_req:RodeoClientRequest, myPlc:Plc, init_evidence:Evidence, env:RodeoEnvironmentMap) -> std::io::Result<ProtocolRunRequest> {

    let top_plc: Plc = myPlc;
    let to_plc: Plc = "P0".to_string();
    
    let asp_id_in: ASP_ID = res_req.RodeoClientReq_attest_id; //"hey".to_string();
    let asp_args_map_in: HashMap<ASP_ID, HashMap<TARG_ID, Value>> = res_req.RodeoClientReq_attest_args;

    let my_env= env.get(&asp_id_in).expect(format!("Term not found in RodeoEnvironmentMap with key: '{}'", asp_id_in).as_str());

    let my_term_orig = my_env.RodeoClientEnv_term.clone();
    let my_term = term_swap_args (my_term_orig, asp_args_map_in, true);
    //let my_term = my_term_noargs;
    let my_session: Attestation_Session = my_env.RodeoClientEnv_session.clone();

    let my_evidence : Evidence = init_evidence;

    let vreq : ProtocolRunRequest = 
    ProtocolRunRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "RUN".to_string(), 
        REQ_PLC: top_plc,
        TO_PLC: to_plc,
        TERM: my_term,
        EVIDENCE: my_evidence,
        ATTESTATION_SESSION: my_session};

    Ok (vreq)
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

fn main() -> std::io::Result<()> {

    let args = get_rodeo_client_args()?;

    let res_req_filepath : String = args.req_filepath;
    println!("\nres_req_filepath arg: {}", res_req_filepath);

    let att_server_uuid_string : String = args.server_uuid.clone();
    println!("server_uuid arg: {}", att_server_uuid_string);

    let client_uuid_string : String = args.client_uuid.clone();
    println!("client_uuid arg: {}", client_uuid_string);

    let res_env_filepath : String = args.env_filepath;
    println!("res_env_filepath arg: {}", res_env_filepath);

    let res_req_contents = fs::read_to_string(res_req_filepath).expect("Couldn't read RodeoClientRequest JSON file");
    eprintln!("\nRodeoClientRequest contents:\n{res_req_contents}");

    let res_req : RodeoClientRequest = serde_json::from_str(&res_req_contents)?;
    println!("\nDecoded RodeoClientRequest as:");
    println!("{:?}", res_req); // :? notation since formatter uses #[derive(..., Debug)] trait

    let res_env_contents = fs::read_to_string(res_env_filepath).expect("Couldn't read res_env JSON file");

    let my_res_env: RodeoEnvironmentMap = serde_json::from_str(&res_env_contents)?;
    eprintln!("\nDecoded res_env as:");
    eprintln!("{:?}", my_res_env);

    let myPlc: Plc = "TOP_PLC".to_string();
    let my_evidence: Evidence = rust_am_lib::copland::EMPTY_EVIDENCE.clone();

    let vreq : ProtocolRunRequest = rodeo_to_am_request(res_req.clone(), myPlc, my_evidence, my_res_env.clone())?;

    let req_str = serde_json::to_string(&vreq)?;

    println!("\nTrying to send ProtocolRunRequest: \n");
    println!("{req_str}\n");

    let resp_str = am_sendRec_string_all(att_server_uuid_string, client_uuid_string, req_str)?;
    eprintln!("Got a TCP Response String: \n");
    eprintln!("{resp_str}\n");

    let resp : ProtocolRunResponse = serde_json::from_str(&resp_str)?;
    println!("Decoded ProtocolRunResponse: \n");
    println!("{:?}\n", resp);

    let asp_id_in: ASP_ID = res_req.RodeoClientReq_attest_id; //"hey".to_string();
    //let asp_args_map_in: HashMap<ASP_ID, HashMap<TARG_ID, Value>> = res_req.RodeoClientReq_attest_args.clone();

    let my_res_env2 = my_res_env.clone();
    let my_env= my_res_env2.get(&asp_id_in).expect(format!("Term not found in RodeoEnvironmentMap with key: '{}'", asp_id_in).as_str());

    let my_att_session = my_env.RodeoClientEnv_session.clone();
    //let app_bool = true;
    let maybe_app_server = args.d_appraisal_server_uuid; //"127.0.0.1:5000".to_string();


    let a_resp = 
        match maybe_app_server {
    //if app_bool {
            Some(app_server) => 
            {
                let app_asp_args = res_req.RodeoClientReq_appraise_args.clone();
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
        _ => {resp.clone()}
    };
    //else { resp.clone() };

    let appsumm_req : AppraisalSummaryRequest = 
    AppraisalSummaryRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "APPSUMM".to_string(), 
        ATTESTATION_SESSION: vreq.ATTESTATION_SESSION.clone(), /* my_att_session.clone(), */
        EVIDENCE: a_resp.PAYLOAD.clone()
    };

    let appsumm_req_str: String = serde_json::to_string(&appsumm_req)?;

    let appsumm_resp_str = am_sendRec_string_all(args.server_uuid.clone(), args.client_uuid.clone(), appsumm_req_str)?;
    println!("Got a TCP Response String: \n");
    println!("{appsumm_resp_str}\n");

    let appsumm_resp : AppraisalSummaryResponse = serde_json::from_str(&appsumm_resp_str)?;
    eprintln!("Decoded AppraisalSummaryResponse: \n");
    eprintln!("{:?}\n", appsumm_resp);


    print_appsumm(appsumm_resp.PAYLOAD, appsumm_resp.SUCCESS);

    let success_bool: bool = appsumm_resp.SUCCESS; //do_response_app_summary(resp.clone());

    let res_resp: RodeoClientResponse = 
        RodeoClientResponse {
            RodeoClientResult_success: success_bool,
            RodeoClientResult_error: "".to_string(),
            RodeoClientResult_term: vreq.TERM,
            RodeoClientResult_evidence: resp.PAYLOAD.clone()
        };

    println!("RodeoClientResponse (Overall Appraisal Success): \n");
    println!("{:?}\n", res_resp.RodeoClientResult_success);

    Ok (())

}







/*


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


    let appsumm_req : AppraisalSummaryRequest = 
    AppraisalSummaryRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "APPSUMM".to_string(), 
        ATTESTATION_SESSION: vreq.ATTESTATION_SESSION.clone(), /* my_att_session.clone(), */
        EVIDENCE: resp.PAYLOAD.clone()
    };

    let appsumm_req_str: String = serde_json::to_string(&appsumm_req)?;

    let appsumm_stream = connect_tcp_stream(args.server_uuid.clone(), args.client_uuid.clone()).await?;
    println!("\nTrying to send AppraisalSummaryRequest: \n");
    println!("{appsumm_req_str}\n");

    let appsumm_resp_str = am_sendRec_string(appsumm_req_str,appsumm_stream).await?;
    println!("Got a TCP Response String: \n");
    println!("{appsumm_resp_str}\n");

    let appsumm_resp : AppraisalSummaryResponse = serde_json::from_str(&appsumm_resp_str)?;
    eprintln!("Decoded AppraisalSummaryResponse: \n");
    eprintln!("{:?}\n", appsumm_resp);


    print_appsumm(appsumm_resp.PAYLOAD, appsumm_resp.SUCCESS);

    let success_bool: bool = appsumm_resp.SUCCESS; //do_response_app_summary(resp.clone());

    let res_resp: RodeoClientResponse = 
        RodeoClientResponse {
            RodeoClientResult_success: success_bool,
            RodeoClientResult_error: "".to_string(),
            RodeoClientResult_term: vreq.TERM,
            RodeoClientResult_evidence: resp.PAYLOAD
        };

    println!("RodeoClientResponse (Overall Appraisal Success): \n");
    println!("{:?}\n", res_resp.RodeoClientResult_success);

    Ok::<(), Error> (())

    }; // end let val = async

    let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(val) {
        Ok(x) => x,
        Err(_) => println!("Runtime failure in rust-rodeo-client main.rs"),
    };
    Ok (())
}

*/


/*
fn do_response_app_summary(resp:ProtocolRunResponse) -> bool {
    let resp_evidence: Evidence = resp.PAYLOAD;
    let resp_rawev_wrapped: RawEv = resp_evidence.RAWEV;
    let resp_rawev: Vec<String> = match resp_rawev_wrapped {
        RawEv::RawEv(rawevt) => rawevt
    };
    let v = resp_rawev.iter().all(|x| *x == "".to_string());
    v
}
    */