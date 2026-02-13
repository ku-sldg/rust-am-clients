// main.rs (rust-hamr-client)

use lib::hamrLib::*;
use lib::clientArgs::*;

use std::fs;


fn main() -> std::io::Result<()> {

    let args = get_rodeo_hamr_client_args()?;

    let attestation_report_root = args.attestation_root;
    let golden_evidence_fp_string = args.golden_evidence_filepath;

    let golden_evidence_fp = std::path::Path::new(&golden_evidence_fp_string);

    let attestation_report_root_fp = std::path::Path::new(&attestation_report_root);

    let default_report_fp = std::path::Path::new("aadl_attestation_report.json");
    let term = do_hamr_term_gen(attestation_report_root_fp, default_report_fp, false, false, false, golden_evidence_fp)?;

    let term_string = serde_json::to_string(&term)?;

    let output_term_fp = args.output_term_filepath.clone();
    fs::write(output_term_fp.clone(), term_string)?;
    eprintln!("\nNew protocol term output to file: {:?} \n\n\n", output_term_fp);

    Ok (())

}

