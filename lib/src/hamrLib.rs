#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use serde_json::json;

use std::collections;
// Other packages required to perform specific ASP action.
use std::fs;
use std::env;
use std::path::{self, Path};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use serde_json::{Value, from_value};
use serde_stacker::Deserializer;


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

    let targmap: std::collections::hash_map::Values<'_, String, std::collections::HashMap<String, AppSummReportValue>>= appsumm.values();
    let targvec: Vec<(&String, &AppSummReportValue)>  = targmap.flatten().collect();
    let resvec: Vec<Resolute_Appsumm_Member> = targvec.into_iter().map(|(x, y)| appsumm_report_value_to_resolute_appsumm_member((*x).clone(),(*y).clone())).collect();

    resvec

}

pub fn appsumm_response_to_resolute_appsumm_response(resp:AppraisalSummaryResponse) -> ResoluteAppraisalSummaryResponse { 

    let appsumm = resp.PAYLOAD;

    let members = appsumm_to_resolute_appsumms(appsumm);
         
    let res : ResoluteAppraisalSummaryResponse = ResoluteAppraisalSummaryResponse {
        TYPE: resp.TYPE,
        ACTION: resp.ACTION,
        SUCCESS: resp.SUCCESS,
        APPRAISAL_RESULT: resp.APPRAISAL_RESULT,
        PAYLOAD: members
    };
    res
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

pub fn HAMR_attestation_report_to_File_Slices (hamr_report:HAMR_AttestationReport, golden_evidence_fp: String, project_root_fp:String) -> Vec<File_Slice> {

    let reports = hamr_report.reports;

    let res1 : Vec<Vec<File_Slice>> = reports.iter().map(|x| HAMR_component_report_to_File_Slices(x.clone(), golden_evidence_fp.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();
    res
}


/*
pub fn HAMR_attestation_report_to_MAESTRO_Slice_ASPs (hamr_report:HAMR_AttestationReport, golden_evidence_fp: String, project_root_fp:String) -> Vec<File_Slice> {

    let reports = hamr_report.reports;

    let res1 : Vec<Vec<File_Slice>> = reports.iter().map(|x| HAMR_component_report_to_File_Slices(x.clone(), golden_evidence_fp.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();
    res
}
    */

fn HAMR_component_report_to_File_Slices (hamr_component_report:HAMR_ComponentReport, golden_evidence_fp: String, project_root_fp:String) -> Vec<File_Slice> {


    let reports = hamr_component_report.reports;
    let idpath = hamr_component_report.idPath;

    let res1 : Vec<Vec<File_Slice>> = reports.iter().map(|x| HAMR_component_contract_report_to_File_Slice(x.clone(), idpath.clone(), golden_evidence_fp.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();

    res
}

/*
fn HAMR_component_report_to_MAESTRO_Slice_ASPs (hamr_component_report:HAMR_ComponentReport, golden_evidence_fp: String, project_root_fp:String) -> Vec<rust_am_lib::copland::ASP> {


    let reports = hamr_component_report.reports;
    let idpath = hamr_component_report.idPath;

    let res1 : Vec<Vec<rust_am_lib::copland::ASP>> = reports.iter().map(|x| HAMR_component_contract_report_to_MAESTRO_Slice_ASPs(x.clone(), idpath.clone(), golden_evidence_fp.clone(), project_root_fp.clone())).collect();

    let res = res1.into_iter().flatten().collect();

    res
}
    */

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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ASP_ARGS_ReadfileRangeMany {
    env_var_golden: String,
    filepath_golden: String,
    outdir: String,
    slices: Vec<File_Slice>
}

fn File_Slice_to_ASP (file_slices:Vec<File_Slice>, golden_evidence_fp:String, outdir_in:String) -> rust_am_lib::copland::ASP {

        let asp_args : ASP_ARGS_ReadfileRangeMany = ASP_ARGS_ReadfileRangeMany 
                { 
                  env_var_golden: "".to_string(),
                  filepath_golden: golden_evidence_fp,
                  outdir: outdir_in,
                  slices: file_slices
                };

        let asp_args_json = serde_json::to_value(asp_args).unwrap();

        //let targid_prefix = maestro_slice.targid;
        //let targid_suffix = maestro_slice.hamr_slice.meta;
        //let targid = targid_prefix; //format!("{targid_prefix}:  {targid_suffix}");

        let slice_asp_params : rust_am_lib::copland::ASP_PARAMS = rust_am_lib::copland::ASP_PARAMS {
        ASP_ID: "readfile_range_many".to_string(),
        ASP_ARGS: asp_args_json,
        ASP_PLC: "P0".to_string(),
        ASP_TARG_ID: "readfile_range_many_targ".to_string()

    };

    rust_am_lib::copland::ASP::ASPC(slice_asp_params)

}

/*
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
    */

fn HAMR_component_contract_report_to_File_Slice (hamr_component_contract_report:HAMR_ComponentContractReport, idpath:Vec<String>, golden_evidence_fp: String, project_root_fp:String) -> Vec<File_Slice> {


    let slices = hamr_component_contract_report.slices;

    let idpath_string = idpath.join("::");
    let component_contract_id = hamr_component_contract_report.id;
    let my_id = format!("{idpath_string}:: {component_contract_id}");

    //eprintln!("\n\n\n\n\n\nMY ID: {}\n\n\n\n\n\n", my_id);

    //panic!("hi");

    let file_slices : Vec<File_Slice> = slices.iter().map(|x| HAMR_Slice_to_File_Slice(x, project_root_fp.clone(), my_id.clone())).collect();

    //let asps : Vec<rust_am_lib::copland::ASP> = maestro_slices.iter().map(|x| MAESTRO_Slice_to_ASP(x.clone(), golden_evidence_fp.clone())).collect();

    file_slices

}

/*
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
    */

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

/*
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
*/
#[derive(Serialize, Deserialize, Debug, Clone)]
struct File_Slice {
    filepath: String,
    start_index: usize,
    end_index: usize
}
fn HAMR_Slice_to_File_Slice (hamr_slice:&HAMR_Slice, project_root_fp:String, id:String) -> File_Slice {

    let uri_relative = hamr_slice.pos.uri.clone();
    //let hamr_kind = hamr_slice.kind.clone();

    let uri_absolute = relpath_to_abspath(project_root_fp, uri_relative);
    let bline = hamr_slice.pos.beginLine;
    let eline = hamr_slice.pos.endLine;

    
    /*
    let bline_string= bline.to_string();
    let eline_string = eline.to_string();
    let uri_slice_string = format!("{uri_absolute}:: {bline_string}-{eline_string}");
    */

    /*
    let uri_slice_string = format!("{uri_absolute}::{bline_string}-{eline_string}");

    //let new_id = format!("{id}:: {uri_slice_string}");

    let res_appsumm_member: Resolute_Appsumm_Member = Resolute_Appsumm_Member 
        { component: (), contract_id: , location: uri_slice_string, meta: hamr_slice.meta, result: false };

    */

    let res : File_Slice = 
        File_Slice { filepath: uri_absolute, 
                        start_index: bline, 
                        end_index: eline };
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
}

pub fn write_string_to_output_dir (maybe_out_dir:Option<String>, fp_suffix: String, default_mid_path:String, outstring:String) -> std::io::Result<String> {

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

pub const DEFAULT_TEMP_COMP_MAP_FILENAME: &'static str = "comp_map_temp.json";
type Slices_Comp_Map = std::collections::HashMap<String, bool>;


fn merge_resolute_slice (root_fp:String, c:HAMR_Slice, m:Slices_Comp_Map, component_id:String, contract_id:String, hm: &mut std::collections::HashMap<String, Resolute_Appsumm_Member>) -> () {

    let pos: HAMR_Pos = c.pos.clone();
    let relative_uri = pos.uri;

    let uri_absolute = relpath_to_abspath(root_fp, relative_uri);
    let bline = c.pos.beginLine;
    let eline = c.pos.endLine;

    let bline_string= bline.to_string();
    let eline_string = eline.to_string();
    let uri_slice_string = format!("{uri_absolute}::{bline_string}-{eline_string}");

    let r = m.get(&uri_slice_string).unwrap();

    let v: Resolute_Appsumm_Member = 
        Resolute_Appsumm_Member 
            { component: component_id, contract_id: contract_id, location: uri_slice_string.clone(), meta: c.meta, result: *r };
    
    hm.entry(uri_slice_string).or_insert(v);

}


fn merge_resolute_contract (root_fp:String, c:HAMR_ComponentContractReport, m:Slices_Comp_Map, component_id:String, hm: &mut std::collections::HashMap<String, Resolute_Appsumm_Member>) -> () {


    let slices = c.slices;

    let _ : Vec<()> = slices.iter().map(|x| merge_resolute_slice (root_fp.clone(), x.clone(), m.clone(), component_id.clone(), c.id.clone(), &mut (*hm))).collect();
}

fn merge_resolute_component (root_fp:String, c:HAMR_ComponentReport, m: &Slices_Comp_Map, hm: &mut std::collections::HashMap<String, Resolute_Appsumm_Member>) -> () {

    let reports = c.reports;

    let idpath_string = c.idPath.join("::");

    let _ : Vec<()> = reports.iter().map(|x| merge_resolute_contract (root_fp.clone(), x.clone(), m.clone(), idpath_string.clone(), &mut (*hm))).collect();
}

fn merge_resolute_appsumm (root_fp:String, r:HAMR_AttestationReport, m:&Slices_Comp_Map) -> ResoluteAppraisalSummaryResponse {

    let mut hm : std::collections::HashMap<String,Resolute_Appsumm_Member> = std::collections::HashMap::new();

    let reports = r.reports;

    let _ : Vec<()> = reports.iter().map(|x| merge_resolute_component(root_fp.clone(), x.clone(), m, &mut hm)).collect();

    //let vals = res1.values();

    let targvec:  Vec<Resolute_Appsumm_Member>   = hm.into_values().collect();

     let mut res_bool = true;

    for v in targvec.clone().into_iter() {
        if ! v.result {res_bool = false; break}
    }


    let res : ResoluteAppraisalSummaryResponse = 
        ResoluteAppraisalSummaryResponse 
            { TYPE: "".to_string(), ACTION: "".to_string(), SUCCESS: true, APPRAISAL_RESULT: res_bool, PAYLOAD: targvec };
    res
}

pub fn generate_resolute_appsumm(hamr_root_dir: String, report_filename: String) -> std::io::Result<ResoluteAppraisalSummaryResponse>  {

    let attestation_report_fp = format!("{hamr_root_dir}/{report_filename}");
    let att_report: HAMR_AttestationReport = get_attestation_report_json(attestation_report_fp.clone())?;

    let comp_map_fp = format!("{hamr_root_dir}/{DEFAULT_TEMP_COMP_MAP_FILENAME}");

    let comp_map_json_string = fs::read_to_string(comp_map_fp)?;

    let comp_map_json = deserialize_deep_json(&comp_map_json_string)?;
    let comp_map : Slices_Comp_Map = serde_json::from_value(comp_map_json)?;

    let resolute_appsumm = merge_resolute_appsumm(hamr_root_dir, att_report, &comp_map);

    Ok(resolute_appsumm)

/*
    let fp_suffix = 
    write_string_to_output_dir(Some(hamr_root_dir.clone()), fp_suffix, "".to_string(), term_string)?;
    



    Ok(())
    */

}

pub fn do_hamr_term_gen(attestation_report_root:String, report_filename: String, hamr_contracts_bool:bool, verus_hash_bool:bool, verus_run_bool:bool, golden_evidence_fp:String) -> std::io::Result<rust_am_lib::copland::Term> {

    let hamr_contracts_bool = 
        if !(hamr_contracts_bool || verus_hash_bool || verus_run_bool)
        { true } // default to ONLY contract checks if nothing specified
        else
        {hamr_contracts_bool};
    
    let mut v: Vec<Term> = Vec::new();

    if hamr_contracts_bool {
        let attestation_report_fp = format!("{attestation_report_root}/{report_filename}");
        let att_report = get_attestation_report_json(attestation_report_fp.clone())?;
        eprintln!("\nDecoded HAMR_AttestationReport: {:?} \n\n\n", att_report.clone());
        

        let slice_vec = HAMR_attestation_report_to_File_Slices(att_report, "".to_string(), attestation_report_root.clone());
        //eprintln!("\nDecoded ASPs vector with size {}: {:?} \n\n\n", asps.len(), asps);


        let asp = File_Slice_to_ASP (slice_vec, golden_evidence_fp, attestation_report_root.clone());



        let term : Term = Term::asp(asp); //ASP_Vec_to_Term(asps);

        let term_string = serde_json::to_string(&term)?;
        let fp_suffix = "hamr_contracts_term.json".to_string();
        write_string_to_output_dir(Some(attestation_report_root.clone()), fp_suffix, "".to_string(), term_string)?;

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
        let fp_suffix = "hamr_verus_hash_term.json".to_string();
        write_string_to_output_dir(Some(attestation_report_root.clone()), fp_suffix, "".to_string(), term_string)?;

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
        let fp_suffix = "hamr_verus_run_term.json".to_string();
        write_string_to_output_dir(Some(attestation_report_root.clone()), fp_suffix, "".to_string(), term_string)?;

        v.push(verus_run_asp_term)
    }

    let contracts_hash_run_term = vec_terms_to_bseq(v.clone());

    let sigparams : ASP_PARAMS = ASP_PARAMS { ASP_ID: "sig".to_string(), ASP_ARGS:json!({}), ASP_PLC: "P0".to_string(), ASP_TARG_ID: "sig_targid".to_string() };
    let sigasp = ASP::ASPC(sigparams);
    let sigterm = Term::lseq(Box::new(contracts_hash_run_term), Box::new(Term::asp(sigasp)));
    eprintln!("\nNew term: {:?} \n\n\n", sigterm);
    Ok(sigterm)

}