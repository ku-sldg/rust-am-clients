#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use serde_json::json;

// Other packages required to perform specific ASP action.
use std::fs;
use std::env;
use std::path::{Path};
use serde::{Deserialize, Serialize};

pub const DEFAULT_HAMR_GOLDEN_EVIDENCE_FILENAME: &'static str = "hamr_contract_golden_evidence.json";

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRangeMany {
    env_var_golden: String,
    filepath_golden: String,
    report_filepath: String,
    attestation_report_filepath: String
}

fn File_Slice_to_ASP (golden_evidence_fp:&Path, report_fp:&Path) -> rust_am_lib::copland::ASP {

        let g_fp = golden_evidence_fp.to_str().unwrap().to_string();
        let r_fp = report_fp.to_str().unwrap().to_string();

        let asp_args : ASP_ARGS_ReadfileRangeMany = ASP_ARGS_ReadfileRangeMany 
                { 
                  env_var_golden: "".to_string(),
                  filepath_golden: g_fp,
                  report_filepath: r_fp.clone(),
                  attestation_report_filepath: r_fp
                };

        let asp_args_json = serde_json::to_value(asp_args).unwrap();

        let slice_asp_params : rust_am_lib::copland::ASP_PARAMS = rust_am_lib::copland::ASP_PARAMS {
        ASP_ID: "hamr_readfile_range_many".to_string(),
        ASP_ARGS: asp_args_json,
        ASP_PLC: "P0".to_string(),
        ASP_TARG_ID: "hamr_readfile_range_many_targ".to_string()

    };

    rust_am_lib::copland::ASP::ASPC(slice_asp_params)

}

fn add_asp (asp:rust_am_lib::copland::ASP, t:rust_am_lib::copland::Term) -> rust_am_lib::copland::Term {

    rust_am_lib::copland::Term::bseq(rust_am_lib::copland::Split {split1:rust_am_lib::copland::SP::ALL, split2:rust_am_lib::copland::SP::ALL},
                                Box::new(t),
                                Box::new(rust_am_lib::copland::Term::asp(asp.clone())) 
                              )

}

fn add_term_bseq (new_t:rust_am_lib::copland::Term, t:rust_am_lib::copland::Term) -> rust_am_lib::copland::Term {

    rust_am_lib::copland::Term::bseq(rust_am_lib::copland::Split {split1:rust_am_lib::copland::SP::ALL, split2:rust_am_lib::copland::SP::ALL},
                                Box::new(t),
                                Box::new(new_t) 
                              )

}

pub fn ASP_Vec_to_Term (asps:Vec<rust_am_lib::copland::ASP>) -> rust_am_lib::copland::Term {

    match asps.as_slice() {

        [] => {rust_am_lib::copland::Term::asp(rust_am_lib::copland::ASP::NULL)}
        [x] => {rust_am_lib::copland::Term::asp(x.clone())}
        [x, _, ..] => 
            {
                let asps_split = asps.split_first().unwrap();
                let rest = asps_split.1.to_vec();
            
                add_asp(x.clone(), ASP_Vec_to_Term(rest))
            }
    }
}

pub fn vec_terms_to_bseq(v:Vec<Term>) -> Term {

    match v.as_slice() {

        [] => {rust_am_lib::copland::Term::asp(rust_am_lib::copland::ASP::NULL)}
        [x] => {x.clone()}
        [x, _, ..] => 
            {
                let terms_split = v.split_first().unwrap();
                let rest = terms_split.1.to_vec();
            
                add_term_bseq(x.clone(), vec_terms_to_bseq(rest))
            }
    }
}

pub fn write_string_to_output_dir (maybe_out_dir:Option<String>, fp_suffix: &Path, default_mid_path:&Path, outstring:String) -> std::io::Result<String> {

    let fp_prefix : String = match maybe_out_dir {
        Some(fp) => {
            fp
        }
        None => {

            let cur_dir = env::current_dir()?;
            let default_path = default_mid_path;
            let default_prefix= cur_dir.join(default_path);
            default_prefix.as_path().to_str().unwrap().to_string()
        }
    };

    let full_req_fp_new = Path::new(&fp_prefix);
    let full_req_fp = full_req_fp_new.join(fp_suffix);

    fs::create_dir_all(fp_prefix)?;
    fs::write(&full_req_fp, outstring)?;
    Ok(full_req_fp.as_path().to_str().unwrap().to_string())
}

pub fn do_hamr_term_gen(maybe_golden_evidence_fp:Option<String>, hamr_report_filepath: &Path, hamr_contracts_bool:bool, verus_hash_bool:bool, verus_run_bool:bool) -> std::io::Result<rust_am_lib::copland::Term> {

    let hamr_contracts_bool = 
        if !(hamr_contracts_bool || verus_hash_bool || verus_run_bool)
        { true } // default to ONLY contract checks if nothing specified
        else
        {hamr_contracts_bool};
    
    let mut v: Vec<Term> = Vec::new();

    let attestation_report_root = hamr_report_filepath.parent().unwrap();

    if hamr_contracts_bool {

        let golden_fp : String  = 
            match maybe_golden_evidence_fp {
                Some(fp) => {
                    fp
                }
                None => {
                    let golden_filepath = attestation_report_root.join(DEFAULT_HAMR_GOLDEN_EVIDENCE_FILENAME);
                    golden_filepath.to_str().unwrap().to_string()
                }
            };
        
        let golden_evidence_fp = Path::new(&golden_fp);

        let asp = File_Slice_to_ASP (golden_evidence_fp, hamr_report_filepath);

        let term : Term = Term::asp(asp);

        let term_string = serde_json::to_string(&term)?;
        let fp_suffix = Path::new("hamr_contracts_only_maestro_term.json");
        write_string_to_output_dir(Some(attestation_report_root.to_str().unwrap().to_string()), fp_suffix, Path::new(""), term_string)?;

        v.push(term)
    }

    if verus_hash_bool {
        let verus_hash_asp_params : ASP_PARAMS = 
                ASP_PARAMS { ASP_ID: "hashfile".to_string(), 
                                ASP_ARGS: json!({}), 
                                ASP_PLC: "p1".to_string(), 
                                ASP_TARG_ID: "cargo_verus_exe_targ".to_string() };
        let verus_hash_asp_term: Term = Term::asp(ASP::ASPC(verus_hash_asp_params));


        let term_string = serde_json::to_string(&verus_hash_asp_term)?;
        let fp_suffix = Path::new("hamr_verus_hash_term.json");
        write_string_to_output_dir(Some(attestation_report_root.to_str().unwrap().to_string()), fp_suffix, Path::new(""), term_string)?;

        v.push(verus_hash_asp_term);
    }
    
    if verus_run_bool {
        let verus_run_asp_params : ASP_PARAMS = 
                        ASP_PARAMS { ASP_ID: "run_command_cargo_verus".to_string(), 
                                    ASP_ARGS: json!({}), 
                                    ASP_PLC: "p1".to_string(), 
                                    ASP_TARG_ID: "run_cargo_verus_targ".to_string() };
        let verus_run_asp_term: Term = Term::asp(ASP::ASPC(verus_run_asp_params));

        let term_string = serde_json::to_string(&verus_run_asp_term)?;
        let fp_suffix = Path::new("hamr_verus_run_term.json");
        write_string_to_output_dir(Some(attestation_report_root.to_str().unwrap().to_string()), fp_suffix, Path::new(""), term_string)?;

        v.push(verus_run_asp_term)
    }

    let contracts_hash_run_term = vec_terms_to_bseq(v.clone());

    let sigparams : ASP_PARAMS = ASP_PARAMS { ASP_ID: "sig".to_string(), ASP_ARGS:json!({}), ASP_PLC: "P0".to_string(), ASP_TARG_ID: "sig_targid".to_string() };
    let sigasp = ASP::ASPC(sigparams);
    let sigterm = Term::lseq(Box::new(contracts_hash_run_term), Box::new(Term::asp(sigasp)));
    eprintln!("\nNew term: {:?} \n\n\n", sigterm);
    Ok(sigterm)

}