// main.rs (rust-rodeo-client)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use lib::clientArgs::*;
use lib::hamrLib::*;

// Other packages required to perform specific ASP action.
use std::fs;
use std::env;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use serde_json::{Value, from_value};
use serde_stacker::Deserializer;

use std::process::{Command};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RodeoClientRequest {
    pub RodeoClientRequest_attest_id: String,
    pub RodeoClientRequest_attest_args: HashMap<ASP_ID, HashMap<TARG_ID, serde_json::Value>>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RodeoClientResponse {
    pub RodeoClientResponse_success: bool,
    pub RodeoClientResponse_error: String,
    pub RodeoClientResponse_cvm_request: ProtocolRunRequest, 
    pub RodeoClientResponse_cvm_response: ProtocolRunResponse
}

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

fn add_appr(t:Term) -> Term {
    Term::lseq(Box::new(t), Box::new(Term::asp(ASP::APPR)))
}


#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RodeoSessionConfig {
    pub RodeoSessionConfig_term: Term,
    pub RodeoSessionConfig_plc: Plc,
    pub RodeoSessionConfig_evidence: Evidence,
    pub RodeoSessionConfig_attest_args: HashMap<ASP_ID, HashMap<TARG_ID, serde_json::Value>>,
    pub RodeoSessionConfig_session: Attestation_Session,
    pub RodeoSessionConfig_appr_flag: bool
}

fn rodeo_to_am_request(rodeo_config: RodeoSessionConfig) -> std::io::Result<ProtocolRunRequest> {

    let RodeoSessionConfig {RodeoSessionConfig_term: my_term_orig, 
                            RodeoSessionConfig_evidence: my_evidence,
                            RodeoSessionConfig_appr_flag: appr_bool,
                            RodeoSessionConfig_attest_args: asp_args_map_in,
                            RodeoSessionConfig_plc: top_plc,
                            RodeoSessionConfig_session: my_session} = rodeo_config;


    let to_plc: Plc = "P0".to_string();
    let my_term_orig_appr: Term = if appr_bool {add_appr(my_term_orig)}
                                  else {my_term_orig};
    let my_term = term_swap_args (my_term_orig_appr, asp_args_map_in, true);
    let my_term_final: Term = rust_am_lib::copland::add_provisioning_args(my_term);

    let vreq : ProtocolRunRequest = 
    ProtocolRunRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "RUN".to_string(), 
        REQ_PLC: top_plc,
        TO_PLC: to_plc,
        TERM: my_term_final,
        EVIDENCE: my_evidence,
        ATTESTATION_SESSION: my_session};

    Ok (vreq)
}


fn write_string_to_output_dir (maybe_out_dir:Option<String>, fp_suffix: String, default_mid_path:String, outstring:String) -> std::io::Result<String> {

    let fp_prefix : String = match &maybe_out_dir {
        Some(fp) => {
            fp.to_string()
        }
        None => {

            let cur_dir = env::current_dir()?;
            let cur_dir_string = cur_dir.to_str().unwrap();
            let default_path = default_mid_path;
            let default_prefix: String = format!("{cur_dir_string}/{default_path}");
            default_prefix
        }
    };

    let full_req_fp = format!("{fp_prefix}/{fp_suffix}");

    fs::create_dir_all(fp_prefix)?;
    fs::write(&full_req_fp, outstring)?;
    Ok(full_req_fp)
}

fn run_cvm_request (cvm_path:String, asp_bin_path:String, manifest_path:String, maybe_out_dir:Option<String>, am_req:ProtocolRunRequest) -> std::io::Result<ProtocolRunResponse> {

    eprintln!("\n\n manifest_path: {}", manifest_path);

    let manifest_contents = fs::read_to_string(manifest_path).expect("Couldn't read Manifest JSON file");
    eprintln!("\nManifest contents:\n{manifest_contents}");

    let am_req_suffix = "cvm_request.json".to_string();
    let am_req_mid_path = "testing/outputs/".to_string();
    let am_req_string = serde_json::to_string(&am_req)?;
    let full_req_fp = write_string_to_output_dir(maybe_out_dir, am_req_suffix, am_req_mid_path, am_req_string.clone())?;

    eprintln!("\n\n\nam_req_string: {:?}\n\n\n", am_req_string);

    let cvm_args = ["--manifest", &manifest_contents, "--asp_bin", &asp_bin_path, "--req_file", &full_req_fp];

    eprintln!("\n\n\nCVM_ARGS: {:?} \n\n\n", cvm_args);

    let output = Command::new(cvm_path)
                                .args(cvm_args).output().expect("error running cvm executable within rust-rodeo-client");

    let err_res : Vec<u8> = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    if ! err_res.is_empty() {eprint!("FYI:  stderr output after invoking cvm in rust-rodeo-client: \n {:?}\n", String::from_utf8(err_res))}

    eprintln!("\n\n\nProtocolRunResponse string: {:?} \n\n\n", String::from_utf8(out_res.clone()));


    let resp_string = String::from_utf8(out_res.clone()).unwrap();


    let respval = deserialize_deep_json(&resp_string)?;

    let resp: Result<ProtocolRunResponse, serde_json::Error> = from_value(respval);

    //let resp : Result<ProtocolRunResponse, serde_json::Error> = serde_json::from_slice(&out_res);
    match resp {

        Ok(v) => {return Ok(v)}
        _ => {panic!("Error decoding ProtocolRunResponse from cvm executable in run_cvm_request (via rust-rodeo-client)")}

    }

}

fn run_appsumm_request (appsumm_req:AppraisalSummaryRequest, appsumm_result_bool: bool) -> std::io::Result<AppraisalSummaryResponse> {


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
                    APPRAISAL_RESULT: appsumm_result_bool,
                    PAYLOAD: s
                };
            appsumm_resp
        } 
        _ => {
                let appsumm_resp : AppraisalSummaryResponse = 
                    AppraisalSummaryResponse {
                        TYPE: "RESPONSE".to_string(), 
                        ACTION: "APPSUMM".to_string(), 
                        SUCCESS: false,
                        APPRAISAL_RESULT: false, // appsumm_result_bool,
                        PAYLOAD: HashMap::new()
                    };
                    appsumm_resp  
        } 

    };

    Ok(summ)

}

fn appsumm_rawev (rev:RawEv) -> bool {

    let inner_rawev = match rev {
        RawEv::RawEv(v) => v
    };

    let result = inner_rawev.iter().all(|x| x == "" ); 

    result
}

fn decode_from_file_and_print<T: DeserializeOwned + std::fmt::Debug + Clone>(term_fp:String, type_string:String) -> Result<T, serde_json::Error> {

    let err_string = format!("Couldn't read {type_string} JSON file");
    let term_contents = fs::read_to_string(term_fp).expect(err_string.as_str());
                                
    let termval = deserialize_deep_json(&term_contents)?;
    let term : T = from_value(termval)?;
    
    eprintln!("\nDecoded term as:");
    eprintln!("{:?}", term);
    Ok(term)
}

fn deserialize_deep_json(json_data: &str) -> serde_json::Result<Value> {
    let mut de = serde_json::de::Deserializer::from_str(json_data);
    de.disable_recursion_limit(); // This method is only available with the feature
    
    // Wrap with serde_stacker's Deserializer to use a dynamically growing stack
    let stacker_de = Deserializer::new(&mut de);
    
    // Deserialize the data
    let value = Value::deserialize(stacker_de)?;
    
    Ok(value)
}

pub const DEFAULT_SESSION_FILENAME: &'static str = "rodeo_configs/sessions/session_union.json";
pub const DEFAULT_HAMR_GOLDEN_EVIDENCE_FILENAME: &'static str = "hamr_contract_golden_evidence.json";
pub const DEFAULT_HAMR_TERM_FILENAME: &'static str = "hamr_contract_term.json";

pub fn rodeo_client_args_to_rodeo_config(args: RodeoClientArgs) -> std::io::Result<RodeoSessionConfig > {

    let session_fp : String = 
        match args.session_filepath {
            Some(fp) => {fp}
            None => {
                let cur_dir = env::current_dir()?;
                let cur_dir_string = cur_dir.to_str().unwrap();
                let default_fp: String = format!("{cur_dir_string}/{DEFAULT_SESSION_FILENAME}");
                default_fp
            }
        };

    let asp_args_map : HashMap<String, HashMap<String, Value>> = 
        match args.g_asp_args_filepath {
            Some(fp) => {
                let asp_args_map: HashMap<String, HashMap<String, Value>> = decode_from_file_and_print(fp, "ASP ARGS MAP".to_string())?;
                asp_args_map
            }
            None => {
                HashMap::new()
            }
        };

    let session = decode_from_file_and_print(session_fp, "Attestation Session".to_string())?;


    let (my_term, my_session, my_asp_args)
            :(Term, Attestation_Session, HashMap<ASP_ID, HashMap<TARG_ID, serde_json::Value>>) 
                = match args.term_filepath {
                    Some(term_fp) => {
                        let err_string = format!("Couldn't read Term JSON file");
                        let term_contents = fs::read_to_string(term_fp).expect(err_string.as_str());

                        let termval = deserialize_deep_json(&term_contents)?;
                        let term : Term = from_value(termval)?;
                        eprintln!("\nDecoded term as:");
                        eprintln!("{:?}", term);

                        (term, session, asp_args_map)
                    }
                    None => { // No Term filepath passed on CLI

                        match args.hamr_root {
                            Some(vec) => {

                                match &vec[..] { // Borrow the Vec as a slice
                                    [hamr_root_dir] => {
                                        let golden_fp = format!("{hamr_root_dir}/{DEFAULT_HAMR_GOLDEN_EVIDENCE_FILENAME}");
                                        let term = do_hamr_term_gen(hamr_root_dir.to_string(), golden_fp)?;

                                        let term_fp = format!("{hamr_root_dir}/{DEFAULT_HAMR_TERM_FILENAME}");
                                        let term_string = serde_json::to_string(&term)?;
                                        fs::write(term_fp, term_string)?;
                                        (term, session, asp_args_map)
                                    },
                                    [hamr_root_dir, golden_filename] => {
                                        let golden_fp = format!("{hamr_root_dir}/{golden_filename}");
                                        let term = do_hamr_term_gen(hamr_root_dir.to_string(), golden_fp)?;

                                        let term_fp = format!("{hamr_root_dir}/{DEFAULT_HAMR_TERM_FILENAME}");
                                        let term_string = serde_json::to_string(&term)?;
                                        fs::write(term_fp, term_string)?;
                                        (term, session, asp_args_map)
                                        
                                    },
                                    /*
                                    [hamr_root_dir, golden_filename, protocol_filename] => {
                                        //println!("The vector has at least three elements. First three are: {}, {}, {}", first, second, third);
                                    }
                                    */
                                    _ => {panic!("hamr_root CLI arg given wrong number of arguments...")}
                                }
            
                            }
                            None => {
                                panic!("Invalid arguments usage for rust-rodeo-client executable:  Must provide either (Term(-t), [Attestation_Session(-s)], [ASP_ARGS Map(-g)]) or (--hamr-root) args!")
                            }
                        } 
                    }                          
    };


    let myPlc: Plc = "TOP_PLC".to_string();
    let my_evidence: Evidence = rust_am_lib::copland::EMPTY_EVIDENCE.clone();

    let appr_bool = args.appraisal;

    Ok (RodeoSessionConfig 
        { RodeoSessionConfig_term: my_term, 
          RodeoSessionConfig_plc: myPlc, 
          RodeoSessionConfig_evidence: my_evidence, 
          RodeoSessionConfig_attest_args: my_asp_args, 
          RodeoSessionConfig_session: my_session, 
          RodeoSessionConfig_appr_flag: appr_bool })

}

fn main() -> std::io::Result<()> {

    let args = get_rodeo_client_args()?;

    let rodeo_session_config = rodeo_client_args_to_rodeo_config(args.clone())?;

    let vreq : ProtocolRunRequest = rodeo_to_am_request(rodeo_session_config.clone())?;

    // Check for "provisinoing mode"
    let maybe_provisioning_flag = 
        match args.provisioned_evidence_filepath {
            Some(golden_fp) => {Some(golden_fp)}
            None => {

                match args.hamr_root {
                    Some(vec) => {

                    match &vec[..] { // Borrow the Vec as a slice
                        [hamr_root_dir] => {
                            let golden_fp = format!("{hamr_root_dir}/{DEFAULT_HAMR_GOLDEN_EVIDENCE_FILENAME}");
                            Some(golden_fp)
                        },
                        [hamr_root_dir, golden_filename] => {
                            let golden_fp = format!("{hamr_root_dir}/{golden_filename}");
                            Some(golden_fp)
                        },
                        /*
                        [hamr_root_dir, golden_filename, protocol_filename] => {
                            //println!("The vector has at least three elements. First three are: {}, {}, {}", first, second, third);
                        }
                        */
                        _ => {panic!("hamr_root CLI arg given wrong number of arguments...")} //println!("The vector has some other number of elements."),
                    }
                    }
                    None => {None}
                }
            }

        };

    let new_vreq = 
        match maybe_provisioning_flag.clone() {
            None => {vreq.clone()}
            Some(prov_filepath) => {

                let new_term: Term = 
                    rust_am_lib::copland::append_provisioning_term(&prov_filepath, 
                                                                    &vreq.REQ_PLC, 
                                                              &vreq.EVIDENCE.1, 
                                                             &vreq.TERM, 
                                                              &vreq.ATTESTATION_SESSION.Session_Context,
                                                                    vreq.TERM.clone());
                let term_string = serde_json::to_string(&new_term)?;
                print!("\n\nProvisioning Term: {}\n\n", term_string);

                ProtocolRunRequest {
                    TERM: new_term,
                    ..vreq.clone()
                }
            }

        };

    let res_cvm_filepath : String = args.cvm_filepath;
    eprintln!("res_cvm_filepath arg: {}", res_cvm_filepath);

    let res_asp_libs_filepath : String = args.libs_asp_bin;
    eprintln!("res_asp_libs_filepath arg: {}", res_asp_libs_filepath);

    let maybe_manifest_fp = args.manifest_filepath;

    let res_manifest_filepath : String = 
        match maybe_manifest_fp {

            Some(fp) => {fp}
            None => {
                let cur_dir = env::current_dir()?;
                let cur_dir_string = cur_dir.to_str().unwrap();
                let default_manifest_fp: String = "testing/manifests/Manifest_P0.json".to_string();
                let default_fp: String = format!("{cur_dir_string}/{default_manifest_fp}");
                default_fp
            }
        };
    eprintln!("res_manifest_filepath arg: {}", res_manifest_filepath);

    let maybe_out_dir = args.output_dir;

    let resp : ProtocolRunResponse = run_cvm_request(res_cvm_filepath, res_asp_libs_filepath, res_manifest_filepath, maybe_out_dir.clone(), new_vreq)?;


    let am_resp_suffix = "cvm_response.json".to_string();
    let am_resp_mid_path = "testing/outputs/".to_string();
    let am_resp_string = serde_json::to_string(&resp)?;
    let _ = write_string_to_output_dir(maybe_out_dir.clone(), am_resp_suffix, am_resp_mid_path, am_resp_string.clone())?;

    let resp_rawev = resp.PAYLOAD.clone().0;
    let success_bool: bool = appsumm_rawev(resp_rawev);

    let res_resp: RodeoClientResponse = 
        RodeoClientResponse {
            RodeoClientResponse_success: success_bool,
            RodeoClientResponse_error: "".to_string(),
            RodeoClientResponse_cvm_request: vreq.clone(),
            RodeoClientResponse_cvm_response: resp.clone()
        };

    let rodeo_resp_string = serde_json::to_string(&res_resp)?;
    print!("{}",rodeo_resp_string);


    match maybe_provisioning_flag {

        None => {
            if args.appraisal {
                let appsumm_req = build_appsumm_request(res_resp.clone());

                let appraisal_valid = appsumm_rawev(res_resp.RodeoClientResponse_cvm_response.PAYLOAD.0);

                let appsumm_resp : AppraisalSummaryResponse = run_appsumm_request(appsumm_req, appraisal_valid)?;
                eprintln!("\n\nDecoded AppraisalSummaryResponse: \n");
                eprintln!("{:?}\n", appsumm_resp);

                eprint_appsumm(appsumm_resp.PAYLOAD.clone(), appraisal_valid);

                let appsumm_resp_suffix = "appsumm_response.json".to_string();
                let appsumm_resp_mid_path = "testing/outputs/".to_string();
                let appsumm_resp_string = serde_json::to_string(&appsumm_resp)?;
                let _ = write_string_to_output_dir(maybe_out_dir.clone(), appsumm_resp_suffix, appsumm_resp_mid_path, appsumm_resp_string.clone())?;
            }
            else {eprintln!("\n\nProtocol completed successfully!\n\n")}
        }
        Some(fp) => {eprintln!("\n\nProvisioned golden evidence to file:\n\t{}\n", fp)}
    };

    Ok (())

}

fn build_appsumm_request (rodeo_resp:RodeoClientResponse) -> AppraisalSummaryRequest {


    let RodeoClientResponse 
        {RodeoClientResponse_success: success_bool,
            RodeoClientResponse_error: _,
            RodeoClientResponse_cvm_request: vreq,
            RodeoClientResponse_cvm_response: vresp} = rodeo_resp;

    eprintln!("RodeoClientResponse (Overall Appraisal Success): \n {:?}: \n", success_bool);
    eprintln!("RodeoClientResponse_cvm_request: \n {:?}: \n", vreq);
    eprintln!("RodeoClientResponse_cvm_response: \n {:?}: \n", vresp);

    let a_resp : ProtocolRunResponse = vresp;

    let my_att_session = vreq.ATTESTATION_SESSION;

    let appsumm_req : AppraisalSummaryRequest = 
    AppraisalSummaryRequest {
        TYPE: "REQUEST".to_string(), 
        ACTION: "APPSUMM".to_string(), 
        ATTESTATION_SESSION: my_att_session.clone(),
        EVIDENCE: a_resp.PAYLOAD.clone()
    };

    appsumm_req


}