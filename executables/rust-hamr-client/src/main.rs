// main.rs (rust-hamr-client)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

/*

use std::collections::HashMap;

use std::process::{Command};

*/

// Custom package imports
use rust_am_lib::copland::*;
use lib::clientArgs::*;

// Other packages required to perform specific ASP action.
use std::fs;
use std::path::{self, Path};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;


#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_Pos {
    r#type: String,
    uri: String,
    beginLine: usize,
    beginCol: usize,
    endLine: usize,
    endCol: usize,
    offset: usize,
    length: usize
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_Slice {
    r#type: String,
    kind: String,
    meta: String,
    pos: HAMR_Pos
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_ComponentContractReport {
    r#type: String,
    id: String,
    kind: String,
    meta: String,
    slices: Vec<HAMR_Slice>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_ComponentReport {
    r#type: String,
    idPath: Vec<String>,
    classifier: Vec<String>,
    reports: Vec<HAMR_ComponentContractReport>
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct HAMR_AttestationReport {
    r#type: String,
    reports: Vec<HAMR_ComponentReport>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct MAESTRO_Slice {
    hamr_slice: HAMR_Slice,
    uri:String,
    beginLine: usize,
    endLine: usize, 
    targid: String
}





fn decode_from_file_and_print<T: DeserializeOwned + std::fmt::Debug + Clone>(term_fp:String, type_string:String) -> Result<T, serde_json::Error> {

     let err_string = format!("Couldn't read {type_string} JSON file");
     let term_contents = fs::read_to_string(term_fp).expect(err_string.as_str());
                                eprintln!("\n{type_string} contents:\n{term_contents}");
                                let term : T = serde_json::from_str(&term_contents)?;
                                eprintln!("\nDecoded Term as:");
                                eprintln!("{:?}", term);
                                Ok(term)
}

fn get_attestation_report_json (hamr_report_fp:String) -> std::io::Result<HAMR_AttestationReport>  {

    let res: HAMR_AttestationReport = decode_from_file_and_print(hamr_report_fp, "HAMR_AttestationReport".to_string())?;

    Ok (res)
}

fn HAMR_attestation_report_to_MAESTRO_Slice_ASPs (hamr_report:HAMR_AttestationReport, project_root_fp:String) -> Vec<rust_am_lib::copland::ASP> {

    let reports = hamr_report.reports;

    let res1 : Vec<Vec<rust_am_lib::copland::ASP>> = reports.iter().map(|x| HAMR_component_report_to_MAESTRO_Slice_ASPs(x.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();
    res
}

fn HAMR_component_report_to_MAESTRO_Slice_ASPs (hamr_component_report:HAMR_ComponentReport, project_root_fp:String) -> Vec<rust_am_lib::copland::ASP> {


    let reports = hamr_component_report.reports;

    let res1 : Vec<Vec<rust_am_lib::copland::ASP>> = reports.iter().map(|x| HAMR_component_contract_report_to_MAESTRO_Slice_ASPs(x.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();

    res
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRange {
    filepath: String,
    start_index: usize,
    end_index: usize, 
    metadata: String, 
    env_var_golden: String,
    filepath_golden: String
}

fn MAESTRO_Slice_to_ASP (maestro_slice:MAESTRO_Slice) -> rust_am_lib::copland::ASP {

        let asp_args : ASP_ARGS_ReadfileRange = ASP_ARGS_ReadfileRange 
                { filepath: maestro_slice.uri, 
                  start_index: maestro_slice.beginLine, 
                  end_index: maestro_slice.endLine,
                  metadata: maestro_slice.hamr_slice.meta.clone(), 
                  env_var_golden: "".to_string(),
                  filepath_golden: "/Users/adampetz/Documents/Summer_2025/maestro_repos/rust-am-clients/goldenFiles/hamr_readfile_range_evidence_golden.json".to_string()
                };

        let asp_args_json = serde_json::to_value(asp_args).unwrap();

        let targid_prefix = maestro_slice.targid;
        let targid_suffix = maestro_slice.hamr_slice.meta;
        let targid = format!("{targid_prefix}:  {targid_suffix}");

        let slice_asp_params : rust_am_lib::copland::ASP_PARAMS = rust_am_lib::copland::ASP_PARAMS {
        ASP_ID: "readfile_range".to_string(),
        ASP_ARGS: asp_args_json,
        ASP_PLC: "P0".to_string(),
        ASP_TARG_ID: targid

    };

    rust_am_lib::copland::ASP::ASPC(slice_asp_params)

}

fn HAMR_component_contract_report_to_MAESTRO_Slice_ASPs (hamr_component_contract_report:HAMR_ComponentContractReport, project_root_fp:String) -> Vec<rust_am_lib::copland::ASP> {


    let slices = hamr_component_contract_report.slices;

    let my_id = hamr_component_contract_report.id;

    let maestro_slices : Vec<MAESTRO_Slice> = slices.iter().map(|x| HAMR_Slice_to_MAESTRO_Slice(x, project_root_fp.clone(), my_id.clone())).collect();

    let asps : Vec<rust_am_lib::copland::ASP> = maestro_slices.iter().map(|x| MAESTRO_Slice_to_ASP(x.clone())).collect();

    asps

    /*

    let slice_asp_params : rust_am_lib::copland::ASP_PARAMS = rust_am_lib::copland::ASP_PARAMS {
        ASP_ID: "readfile_range".to_string(),
        ASP_ARGS: serde_json::Value::Null,
        ASP_PLC: "hi".to_string(),
        ASP_TARG_ID: "hi".to_string()

    };

     vec![rust_am_lib::copland::ASP::ASPC(slice_asp_params)]
     */

}

fn relpath_to_abspath (project_root_fp:String, relpath:String) -> String {

    let root = Path::new(&project_root_fp);
    let relative = Path::new(&relpath);

    let combined_path = root.join(relative);
    
    // Normalize the path using std::path::absolute
    let normalized_absolute_path = path::absolute(&combined_path).unwrap();

    let canonnicalized_path = fs::canonicalize(normalized_absolute_path).unwrap();

    let res = canonnicalized_path.to_str().unwrap().to_string();
    res

}


fn HAMR_Slice_to_MAESTRO_Slice (hamr_slice:&HAMR_Slice, project_root_fp:String, id:String) -> MAESTRO_Slice {

    let uri_relative = hamr_slice.pos.uri.clone();
    //let hamr_kind = hamr_slice.kind.clone();

    let uri_absolute = relpath_to_abspath(project_root_fp, uri_relative);
    let bline = hamr_slice.pos.beginLine;
    let eline = hamr_slice.pos.endLine;

    let res : MAESTRO_Slice = 
        MAESTRO_Slice { hamr_slice:hamr_slice.clone(), 
                        uri: uri_absolute, 
                        beginLine: bline, 
                        endLine: eline, 
                        targid: id };
    res
}

fn add_asp (asp:rust_am_lib::copland::ASP, t:rust_am_lib::copland::Term) -> rust_am_lib::copland::Term {

    rust_am_lib::copland::Term::bseq(rust_am_lib::copland::Split {split1:rust_am_lib::copland::SP::ALL, split2:rust_am_lib::copland::SP::ALL},
                                Box::new(t),
                                Box::new(rust_am_lib::copland::Term::asp(asp.clone())) 
                              )

}

fn ASP_Vec_to_Term (asps:Vec<rust_am_lib::copland::ASP>) -> rust_am_lib::copland::Term {

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


fn main() -> std::io::Result<()> {

    //let args = get_hamr_client_args()?;

    
    let attestation_report_fp = "/Users/adampetz/Documents/Summer_2025/maestro_repos/rust-am-clients/executables/rust-hamr-client/test_data/aadl_attestation_report.json".to_string();



    let att_report = get_attestation_report_json(attestation_report_fp)?;
    println!("\nDecoded HAMR_AttestationReport: {:?} \n\n\n", att_report);


    let attestation_root_fp = "/Users/adampetz/Documents/Summer_2025/INSPECTA-models/isolette/hamr/microkit/attestation/".to_string();
    

    let asps = HAMR_attestation_report_to_MAESTRO_Slice_ASPs(att_report, attestation_root_fp.clone());

    println!("\nDecoded ASPs vector with size {}: {:?} \n\n\n", asps.len(), asps);

    let term = ASP_Vec_to_Term(asps);


    println!("\nNew term: {:?} \n\n\n", term);

    let term_string = serde_json::to_string(&term)?;

    let full_fp = "/Users/adampetz/Documents/Summer_2025/maestro_repos/rust-am-clients/testing/hamr_term.json".to_string();
    fs::write(full_fp, term_string)?;







    /*
    let hamr_uri = "../../../aadl/aadl/packages/Regulate.aadl".to_string();

    let hamr_uri2 = "../crates/thermostat_rt_mri_mri/src/component/thermostat_rt_mri_mri_app.rs".to_string();

    let root_path = "/Users/adampetz/Documents/Summer_2025/INSPECTA-models/isolette/hamr/microkit/attestation".to_string();

    let newpath = HAMR_relpath_to_abspath(root_path, hamr_uri2);

    println!("\nNew path: {:?} \n\n", newpath);
    */


    Ok (())

}

