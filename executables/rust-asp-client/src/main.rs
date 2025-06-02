// Common Packages
use anyhow::{Context, Result};
use rust_am_lib::copland::{self, ASPRunRequest, ASPRunResponse, RawEv, ASP_ARGS};

use serde_json::{Value, json};

use std::process::{Command};

use std::str;
use std::fs;

/*

// function where the work of the ASP is performed.
// May signal an error which will be handled in main.
fn body(_ev: copland::ASP_RawEv, args: copland::ASP_ARGS) -> Result<copland::ASP_RawEv> {
    let filename = args
        .get("filepath")
        .context("filepath argument not provided to ASP, r_readfile_id")?;

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

    let bytes = std::fs::read(&filename_full).context(
        "could not read file contents in ASP, r_readfile_id.  Perhaps the file doesn't exits?",
    )?; // Vec<u8>
    Ok(vec![bytes])
}

// Main simply invokes the body() function above,
// and checks for Err Result.
// If it detects an Err Result, this ASP will return
// an ASPRunResponse with SUCCESS = false, o/w uses
// ASPRunResponse returned from body()


*/

fn main() {

     let aspid: String = "".to_string();

     let attestation_args_filepath = "/Users/adampetz/Documents/Spring_2025/my_am_repos/rust-am-clients/testing/asp_args/concretized_args/filehash_args.json".to_string();

    let attestation_args_contents = fs::read_to_string(attestation_args_filepath).expect("Couldn't read Appraisal ASP_ARGS JSON file");
    eprintln!("\nAttestation ASP_ARGS contents:\n{attestation_args_contents}");

    let asp_args :  ASP_ARGS = serde_json::from_str(&attestation_args_contents).expect("hihi");
    eprintln!("\nDecoded Attestation ASP_ARGS as:");
    eprintln!("{:?}", asp_args);
    //eprintln!("\nAttestation ASP_ARGS contents:\n{attestation_args_contents}");





     //let aspargs: Value = json!(null);
     let aspplc: String = "".to_string();
     let asptargid: String = "".to_string();
     let aspinitrawev: RawEv = rust_am_lib::copland::EMPTY_EVIDENCE.RAWEV.clone();
     let asp_req : ASPRunRequest = 
                ASPRunRequest {
                    TYPE: "REQUEST".to_string(), 
                    ACTION: "ASP_RUN".to_string(),
                    ASP_ID: aspid, 
                    ASP_ARGS: asp_args,
                    ASP_PLC: aspplc,
                    ASP_TARG_ID: asptargid,
                    RAWEV: aspinitrawev
                };

    let req_str = serde_json::to_string(&asp_req).expect("hey");            

    let asp_exe_path= "/Users/adampetz/Documents/Spring_2025/my_am_repos/asp-libs/target/release/hashfile".to_string();
    let asp_exe_args: Vec<String> = vec!(req_str);
    let output = Command::new(asp_exe_path)
                                .args(asp_exe_args).output().expect("hi");

    let err_res = output.stderr;
    let out_res : Vec<u8> = output.stdout;

    let res = if err_res.is_empty() {out_res} 
                       else {err_res};
    
    let strres = str::from_utf8(&res).expect("hee");
    
    println!("hihi");

    println!("{strres}");



    
    //handle_body(body);
}
