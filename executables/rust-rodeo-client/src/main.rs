// main.rs (rust-rodeo-client)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use lib::clientArgs::*;

// Other packages required to perform specific ASP action.
use std::fs;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

use std::process::{Command};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RodeoClientRequest {
    pub RodeoClientRequest_attest_id: String,
    pub RodeoClientRequest_attest_args: HashMap<ASP_ID, HashMap<TARG_ID, serde_json::Value>>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RodeoClientResponse {
    pub RodeoClientResponse_success: bool,
    pub RodeoClientResponse_error: String,
    pub RodeoClientResponse_cvm_request: ProtocolRunRequest, 
    pub RodeoClientResponse_cvm_response: ProtocolRunResponse
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RodeoEnvironment {
    pub RodeoClientEnv_term: Term,
    pub RodeoClientEnv_session: Attestation_Session
}

pub type RodeoEnvironmentMap = HashMap<ASP_ID, RodeoEnvironment>;


fn aspc_args_swap(params:ASP_PARAMS, args_map:HashMap<ASP_ID, HashMap<TARG_ID, serde_json::Value>>, keep_orig:bool) -> ASP_PARAMS {

    let id : ASP_ID = params.ASP_ID.clone();
    let targid : TARG_ID = params.ASP_TARG_ID.clone();
    let new_args: serde_json::Value = 
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
                    {serde_json::json!({})}

                }
            }
        }
        None => {
            if keep_orig 
                {params.ASP_ARGS} 
            else 
                {serde_json::json!({})}
        }
        
      };
      
    ASP_PARAMS { 
        ASP_ARGS: new_args,
        ASP_ID: params.ASP_ID,
        ASP_PLC: params.ASP_PLC,
        ASP_TARG_ID: params.ASP_TARG_ID,
    }

}

fn term_swap_args(t:Term, args_map:HashMap<ASP_ID, HashMap<TARG_ID, serde_json::Value>>, keep_orig:bool) -> Term {
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
    
    let asp_id_in: ASP_ID = res_req.RodeoClientRequest_attest_id;
    let asp_args_map_in: HashMap<ASP_ID, HashMap<TARG_ID, serde_json::Value>> = res_req.RodeoClientRequest_attest_args;

    let my_env= env.get(&asp_id_in).expect(format!("Term not found in RodeoEnvironmentMap with key: '{}'", asp_id_in).as_str());

    let my_term_orig = my_env.RodeoClientEnv_term.clone();
    let my_term = term_swap_args (my_term_orig, asp_args_map_in, false);
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

fn run_cvm_request (cvm_path:String, am_req:ProtocolRunRequest) -> std::io::Result<ProtocolRunResponse> {

    const DEFAULT_ASP_BIN_PATH: &'static str = "/asp-libs/target/release/";
    const DEFAULT_MANIFEST_PATH: &'static str = "/rust-am-clients/testing/manifests/Manifest_P0.json";

    let manifest_path = get_local_env_var_w_suffix(lib::clientArgs::AM_REPOS_ROOT_ENV_VAR.to_string(), 
                                   DEFAULT_MANIFEST_PATH).expect(&format!("Couldn't initialize default value for manifest_path inside run_cvm_request().  
                                                                Check for missing Environment Variable: {}", AM_REPOS_ROOT_ENV_VAR));

    let asp_bin_path = get_local_env_var_w_suffix(lib::clientArgs::AM_REPOS_ROOT_ENV_VAR.to_string(), 
                                   DEFAULT_ASP_BIN_PATH).expect(&format!("Couldn't initialize default value for asp_bin_path inside run_cvm_request().  
                                                                Check for missing Environment Variable: {}", AM_REPOS_ROOT_ENV_VAR));

    eprintln!("\n\n manifest_path: {}", manifest_path);

    let manifest_contents = fs::read_to_string(manifest_path).expect("Couldn't read Manifest JSON file");
    eprintln!("\nManifest contents:\n{manifest_contents}");

    let am_req_string = serde_json::to_string(&am_req)?;

    eprintln!("\n\n\nam_req_string: {:?}\n\n\n", am_req_string);

    let cvm_args = ["--manifest", &manifest_contents, "--asp_bin", &asp_bin_path, "--req", &am_req_string];


    eprintln!("\n\n\nCVM_ARGS: {:?} \n\n\n", cvm_args);


    let output = Command::new(cvm_path)
                                .args(cvm_args).output().expect("error running cvm executable within rust-rodeo-client");

    let err_res : Vec<u8> = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    if ! err_res.is_empty() {eprint!("FYI:  stderr output after invoking cvm in rust-rodeo-client: {:?}", String::from_utf8(err_res))}

    eprintln!("\n\n\nProtocolRunResponse string: {:?} \n\n\n", String::from_utf8(out_res.clone()));

    let resp : Result<ProtocolRunResponse, serde_json::Error> = serde_json::from_slice(&out_res);
    match resp {

        Ok(v) => {return Ok(v)}
        _ => {panic!("Error decoding ProtocolRunResponse from cvm executable in run_cvm_request (via rust-rodeo-client)")}

    }

}

fn run_appsumm_request (appsumm_req:AppraisalSummaryRequest) -> std::io::Result<AppraisalSummaryResponse> {


    let et = appsumm_req.EVIDENCE.1.clone();
    let rev = appsumm_req.EVIDENCE.0.clone();
    let rev_t = match rev.clone() { RawEv::RawEv(v) => v };
    let g = appsumm_req.ATTESTATION_SESSION.Session_Context;

    let appsumm_result = do_AppraisalSummary(et, rev_t, g);

    let summ : AppraisalSummaryResponse = match appsumm_result {
        Ok(s) => {

            let appsumm_resp : AppraisalSummaryResponse = 
                AppraisalSummaryResponse {
                    TYPE: "RESPONSE".to_string(), 
                    ACTION: "APPSUMM".to_string(), 
                    SUCCESS: true,
                    PAYLOAD: s
                };
            appsumm_resp
        } 
        _ => { //panic!("do_AppraisalSummary failed in run_appsumm_request")
                let appsumm_resp : AppraisalSummaryResponse = 
                    AppraisalSummaryResponse {
                        TYPE: "RESPONSE".to_string(), 
                        ACTION: "APPSUMM".to_string(), 
                        SUCCESS: false,
                        PAYLOAD: HashMap::new()
                    };
                    appsumm_resp  
        } 

    };

    Ok(summ)



    /*

    const DEFAULT_EVTOOLS_PATH: &'static str = "/copland-evidence-tools/_build/install/default/bin/copland_evidence_tools";

    let evtools_path = get_local_env_var_w_suffix(lib::clientArgs::AM_REPOS_ROOT_ENV_VAR.to_string(), 
                                   DEFAULT_EVTOOLS_PATH).expect(&format!("Couldn't initialize default value for evtools_path inside run_cvm_request().  
                                                                Check for missing Environment Variable: {}", AM_REPOS_ROOT_ENV_VAR));

    let appsumm_req_string = serde_json::to_string(&appsumm_req)?;

    let evtools_args = ["--req", &appsumm_req_string];

    let output = Command::new(evtools_path)
                                .args(evtools_args).output().expect("error running copland-evidence-tools within rust-rodeo-client");

    let err_res = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    if ! err_res.is_empty() {eprint!("FYI:  stderr output after invoking copland-evidence-tools in rust-rodeo-client: {:?}", String::from_utf8(err_res))}

    let resp : Result<AppraisalSummaryResponse, serde_json::Error> = serde_json::from_slice(&out_res);
    match resp {

        Ok(v) => {return Ok(v)}
        _ => {panic!("Error decoding AppraisalSummaryResponse from copland-evidence-tools executable in run_appsumm_request (via rust-rodeo-client)")}

    }
    */
}

fn appsumm_rawev (rev:RawEv) -> bool {

    let inner_rawev = match rev {
        RawEv::RawEv(v) => v
    };

    let result = inner_rawev.iter().all(|x| x == "" ); 

    result
}


fn main() -> std::io::Result<()> {

    let args = get_rodeo_client_args()?;

    let res_req_filepath : String = args.req_filepath;
    eprintln!("\nres_req_filepath arg: {}", res_req_filepath);

    let res_env_filepath : String = args.env_filepath;
    eprintln!("res_env_filepath arg: {}", res_env_filepath);

    let res_cvm_filepath : String = args.cvm_filepath;
    eprintln!("res_cvm_filepath arg: {}", res_cvm_filepath);

    let res_req_contents = fs::read_to_string(res_req_filepath).expect("Couldn't read RodeoClientRequest JSON file");
    eprintln!("\nRodeoClientRequest contents:\n{res_req_contents}");

    let res_req : RodeoClientRequest = serde_json::from_str(&res_req_contents)?;
    eprintln!("\nDecoded RodeoClientRequest as:");
    eprintln!("{:?}", res_req); // :? notation since formatter uses #[derive(..., Debug)] trait

    let res_env_contents = fs::read_to_string(res_env_filepath).expect("Couldn't read res_env JSON file");

    eprintln!{"\n\nAttempting to decode RodeoEnvironmentMap...\n\n"};
    let my_res_env: RodeoEnvironmentMap = serde_json::from_str(&res_env_contents)?;
    eprintln!("\nDecoded res_env as:");
    eprintln!("{:?}", my_res_env);

    let myPlc: Plc = "TOP_PLC".to_string();
    let my_evidence: Evidence = rust_am_lib::copland::EMPTY_EVIDENCE.clone();

    let vreq : ProtocolRunRequest = rodeo_to_am_request(res_req.clone(), myPlc, my_evidence, my_res_env.clone())?;

    // Check for "provisinoing mode"
    let maybe_provisioning_flag = args.provisioned_evidence_filepath;

    let new_vreq = 
        match maybe_provisioning_flag {
            None => {vreq.clone()}
            Some(prov_filepath) => {

                let new_term: Term = 
                    rust_am_lib::copland::append_provisioning_term(&prov_filepath, 
                                                                    &vreq.REQ_PLC, 
                                                              &vreq.EVIDENCE.1, 
                                                             &vreq.TERM, 
                                                                    vreq.TERM.clone());
                let term_string = serde_json::to_string(&new_term)?;
                print!("\n\nProvisioning Term: {}\n\n", term_string);

                ProtocolRunRequest {
                    TERM: new_term,
                    ..vreq.clone()
                }
            }

        };

    let resp : ProtocolRunResponse = run_cvm_request(res_cvm_filepath, new_vreq)?;

    let resp_rawev = resp.PAYLOAD.clone().0;

    let success_bool: bool = appsumm_rawev(resp_rawev);

    let res_resp: RodeoClientResponse = 
        RodeoClientResponse {
            RodeoClientResponse_success: success_bool,
            RodeoClientResponse_error: "".to_string(),
            RodeoClientResponse_cvm_request: vreq.clone(),
            RodeoClientResponse_cvm_response: resp.clone()
        };

    eprintln!("RodeoClientResponse (Overall Appraisal Success): \n");
    eprintln!("{:?}\n", res_resp.RodeoClientResponse_success);

    eprintln!("RodeoClientResponse_cvm_request: \n {:?}: \n", res_resp.RodeoClientResponse_cvm_request);

    eprintln!("RodeoClientResponse_cvm_response: \n {:?}: \n", res_resp.RodeoClientResponse_cvm_response);


    let rodeo_resp_string = serde_json::to_string(&res_resp)?;
    print!("{}",rodeo_resp_string);

    let a_resp : ProtocolRunResponse = res_resp.RodeoClientResponse_cvm_response;

    let asp_id_in: ASP_ID = res_req.RodeoClientRequest_attest_id;

    let my_env= my_res_env.get(&asp_id_in).expect(format!("Term not found in RodeoEnvironmentMap with key: '{}'", asp_id_in).as_str());

    let my_att_session = my_env.RodeoClientEnv_session.clone();

    let appsumm_req : AppraisalSummaryRequest = 
    AppraisalSummaryRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "APPSUMM".to_string(), 
        ATTESTATION_SESSION: my_att_session.clone(),
        EVIDENCE: a_resp.PAYLOAD.clone()
    };

    let appsumm_resp : AppraisalSummaryResponse = run_appsumm_request(appsumm_req)?;
    eprintln!("\n\nDecoded AppraisalSummaryResponse: \n");
    eprintln!("{:?}\n", appsumm_resp);

    let appraisal_valid = appsumm_rawev(a_resp.PAYLOAD.0.clone());

    eprint_appsumm(appsumm_resp.PAYLOAD, appraisal_valid);

    Ok (())

}