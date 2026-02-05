#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use serde_json::json;

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
pub struct HAMR_AttestationReport {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Resolute_Appsumm_Member {
    component:String,
    contract_id:String,
    location:String,
    meta:String,
    result:bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResoluteAppraisalSummaryResponse {
    pub TYPE: String,
    pub ACTION: String,
    pub SUCCESS: bool,
    pub APPRAISAL_RESULT: bool,
    pub PAYLOAD: Vec<Resolute_Appsumm_Member>
}


fn appsumm_report_value_to_resolute_appsumm_member (targid:String, v:AppSummReportValue) -> Resolute_Appsumm_Member {

    let appsumm_targid_string = targid.clone();

    if targid.contains("::") 
    {

    let (component_res_string, rest) = appsumm_targid_string.split_once(":: ").unwrap();
    let (id_res_string, rest) = rest.split_once(":: ").unwrap();
    let (uri_res_string, rest) = rest.split_once(":: ").unwrap();
    let range_res_string = rest;

    let location_res_string = format!("{uri_res_string}::{range_res_string}");

     let res : Resolute_Appsumm_Member = Resolute_Appsumm_Member {
        contract_id: id_res_string.to_string(),
        component: component_res_string.to_string(),
        location: location_res_string.to_string(),
        meta: v.meta,
        result: v.result
     };

     res
    }
    else {
        let res : Resolute_Appsumm_Member = Resolute_Appsumm_Member {
            contract_id: "".to_string(),
            component: "".to_string(),
            location: "".to_string(),
            meta: v.meta,
            result: v.result
        };

     res
    }

}

fn appsumm_to_resolute_appsumms (appsumm:AppraisalSummary) -> Vec<Resolute_Appsumm_Member> {

    //let golden_evidence_appr_key = "goldenevidence_appr";
    let targmap: std::collections::hash_map::Values<'_, String, std::collections::HashMap<String, AppSummReportValue>>= appsumm.values(); //appsumm.get(golden_evidence_appr_key).unwrap();




    /* : Vec<(&String, &AppSummReportValue)> */ 
    let targvec: Vec<(&String, &AppSummReportValue)>  = targmap.flatten().collect();

    let resvec: Vec<Resolute_Appsumm_Member> = targvec.into_iter().map(|(x, y)| appsumm_report_value_to_resolute_appsumm_member((*x).clone(),(*y).clone())).collect();

    resvec

}

pub fn appsumm_response_to_resolute_appsumm_response(resp:AppraisalSummaryResponse) -> ResoluteAppraisalSummaryResponse { 

    let appsumm = resp.PAYLOAD;

    let members = appsumm_to_resolute_appsumms(appsumm);

    //let json_members: Vec<serde_json::Value> = members.iter().map(|x| serde_json::to_value(x).unwrap()).collect();
         
    let res : ResoluteAppraisalSummaryResponse = ResoluteAppraisalSummaryResponse {
        TYPE: resp.TYPE,
        ACTION: resp.ACTION,
        SUCCESS: resp.SUCCESS,
        APPRAISAL_RESULT: resp.APPRAISAL_RESULT,
        PAYLOAD: members
    };
    res
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

pub fn get_attestation_report_json (hamr_report_fp:String) -> std::io::Result<HAMR_AttestationReport>  {

    let res: HAMR_AttestationReport = decode_from_file_and_print(hamr_report_fp, "HAMR_AttestationReport".to_string())?;

    Ok (res)
}

pub fn HAMR_attestation_report_to_MAESTRO_Slice_ASPs (hamr_report:HAMR_AttestationReport, golden_evidence_fp: String, project_root_fp:String) -> Vec<rust_am_lib::copland::ASP> {

    let reports = hamr_report.reports;

    let res1 : Vec<Vec<rust_am_lib::copland::ASP>> = reports.iter().map(|x| HAMR_component_report_to_MAESTRO_Slice_ASPs(x.clone(), golden_evidence_fp.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();
    res
}

fn HAMR_component_report_to_MAESTRO_Slice_ASPs (hamr_component_report:HAMR_ComponentReport, golden_evidence_fp: String, project_root_fp:String) -> Vec<rust_am_lib::copland::ASP> {


    let reports = hamr_component_report.reports;
    let idpath = hamr_component_report.idPath;

    let res1 : Vec<Vec<rust_am_lib::copland::ASP>> = reports.iter().map(|x| HAMR_component_contract_report_to_MAESTRO_Slice_ASPs(x.clone(), idpath.clone(), golden_evidence_fp.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();

    res
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRange {
    filepath: String,
    start_index: usize,
    end_index: usize, 
    metadata: String, 
    meta: String,
    env_var_golden: String,
    filepath_golden: String
}

fn MAESTRO_Slice_to_ASP (maestro_slice:MAESTRO_Slice, golden_evidence_fp:String) -> rust_am_lib::copland::ASP {

        let asp_args : ASP_ARGS_ReadfileRange = ASP_ARGS_ReadfileRange 
                { filepath: maestro_slice.uri, 
                  start_index: maestro_slice.beginLine, 
                  end_index: maestro_slice.endLine,
                  metadata: maestro_slice.hamr_slice.meta.clone(), 
                  meta: maestro_slice.hamr_slice.meta.clone(),
                  env_var_golden: "".to_string(),
                  filepath_golden: golden_evidence_fp
                };

        let asp_args_json = serde_json::to_value(asp_args).unwrap();

        let targid_prefix = maestro_slice.targid;
        //let targid_suffix = maestro_slice.hamr_slice.meta;
        let targid = targid_prefix; //format!("{targid_prefix}:  {targid_suffix}");

        let slice_asp_params : rust_am_lib::copland::ASP_PARAMS = rust_am_lib::copland::ASP_PARAMS {
        ASP_ID: "readfile_range".to_string(),
        ASP_ARGS: asp_args_json,
        ASP_PLC: "P0".to_string(),
        ASP_TARG_ID: targid

    };

    rust_am_lib::copland::ASP::ASPC(slice_asp_params)

}

fn HAMR_component_contract_report_to_MAESTRO_Slice_ASPs (hamr_component_contract_report:HAMR_ComponentContractReport, idpath:Vec<String>, golden_evidence_fp: String, project_root_fp:String) -> Vec<rust_am_lib::copland::ASP> {


    let slices = hamr_component_contract_report.slices;

    let idpath_string = idpath.join("::");
    let component_contract_id = hamr_component_contract_report.id;
    let my_id = format!("{idpath_string}:: {component_contract_id}");

    //eprintln!("\n\n\n\n\n\nMY ID: {}\n\n\n\n\n\n", my_id);

    //panic!("hi");

    let maestro_slices : Vec<MAESTRO_Slice> = slices.iter().map(|x| HAMR_Slice_to_MAESTRO_Slice(x, project_root_fp.clone(), my_id.clone())).collect();

    let asps : Vec<rust_am_lib::copland::ASP> = maestro_slices.iter().map(|x| MAESTRO_Slice_to_ASP(x.clone(), golden_evidence_fp.clone())).collect();

    asps

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

    let bline_string= bline.to_string();
    let eline_string = eline.to_string();
    let uri_slice_string = format!("{uri_absolute}:: {bline_string}-{eline_string}");

    let new_id = format!("{id}:: {uri_slice_string}");

    let res : MAESTRO_Slice = 
        MAESTRO_Slice { hamr_slice:hamr_slice.clone(), 
                        uri: uri_absolute, 
                        beginLine: bline, 
                        endLine: eline, 
                        targid: new_id };
    res
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

    //Term::asp(ASP::NULL)
}

pub fn do_hamr_term_gen(attestation_report_root:String, hamr_contracts_bool:bool, verus_hash_bool:bool, verus_run_bool:bool, golden_evidence_fp:String) -> std::io::Result<rust_am_lib::copland::Term> {

    let hamr_contracts_bool = 
        if !(hamr_contracts_bool || verus_hash_bool || verus_run_bool)
        { true } // default to ONLY contract checks if nothing specified
        else
        {hamr_contracts_bool};
    
    let mut v: Vec<Term> = Vec::new();

    if hamr_contracts_bool {
        let default_report_filename: String = "aadl_attestation_report.json".to_string();
        let attestation_report_fp = format!("{attestation_report_root}/{default_report_filename}");
        let att_report = get_attestation_report_json(attestation_report_fp)?;
        eprintln!("\nDecoded HAMR_AttestationReport: {:?} \n\n\n", att_report);
        

        let asps = HAMR_attestation_report_to_MAESTRO_Slice_ASPs(att_report, golden_evidence_fp, attestation_report_root);
        eprintln!("\nDecoded ASPs vector with size {}: {:?} \n\n\n", asps.len(), asps);

        let term = ASP_Vec_to_Term(asps);
        eprintln!("\nNew term: {:?} \n\n\n", term);

        v.push(term)
    }

    if verus_hash_bool {
        let verus_hash_asp_params : ASP_PARAMS = 
                ASP_PARAMS { ASP_ID: "hashfile".to_string(), 
                                ASP_ARGS: json!({}), 
                                ASP_PLC: "p1".to_string(), 
                                ASP_TARG_ID: "cargo_verus_exe_targ".to_string() };
        let verus_hash_asp_term: Term = Term::asp(ASP::ASPC(verus_hash_asp_params));

        v.push(verus_hash_asp_term);
    }
    
    if verus_run_bool {
        let verus_run_asp_params : ASP_PARAMS = 
                        ASP_PARAMS { ASP_ID: "run_command_cargo_verus".to_string(), 
                                    ASP_ARGS: json!({}), 
                                    ASP_PLC: "p1".to_string(), 
                                    ASP_TARG_ID: "run_cargo_verus_targ".to_string() };
        let verus_run_asp_term: Term = Term::asp(ASP::ASPC(verus_run_asp_params));
        v.push(verus_run_asp_term)
    }

    let contracts_hash_run_term = vec_terms_to_bseq(v);

    let sigparams : ASP_PARAMS = ASP_PARAMS { ASP_ID: "sig".to_string(), ASP_ARGS:json!({}), ASP_PLC: "P0".to_string(), ASP_TARG_ID: "sig_targid".to_string() };
    let sigasp = ASP::ASPC(sigparams);
    let sigterm = Term::lseq(Box::new(contracts_hash_run_term), Box::new(Term::asp(sigasp)));
    eprintln!("\nNew term: {:?} \n\n\n", sigterm);
    Ok(sigterm)

}