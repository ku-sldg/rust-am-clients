// main.rs (rust-hamr-client)

use lib::hamrLib::*;
use lib::clientArgs::*;

use std::fs;


fn main() -> std::io::Result<()> {

    let args = get_rodeo_hamr_client_args()?;

    
    let attestation_report_root = args.attestation_root;
    let attestation_report_fp = format!("{attestation_report_root}/aadl_attestation_report.json");
    let golden_evidence_fp = args.golden_evidence_filepath;

    let att_report = get_attestation_report_json(attestation_report_fp)?;
    eprintln!("\nDecoded HAMR_AttestationReport: {:?} \n\n\n", att_report);
    

    let asps = HAMR_attestation_report_to_MAESTRO_Slice_ASPs(att_report, golden_evidence_fp, attestation_report_root);
    eprintln!("\nDecoded ASPs vector with size {}: {:?} \n\n\n", asps.len(), asps);

    let term = ASP_Vec_to_Term(asps);
    eprintln!("\nNew term: {:?} \n\n\n", term);

    let term_string = serde_json::to_string(&term)?;

    let output_term_fp = args.output_term_filepath.clone();
    fs::write(output_term_fp.clone(), term_string)?;
    eprintln!("\nNew protocol term output to file: {:?} \n\n\n", output_term_fp);

    Ok (())

}

