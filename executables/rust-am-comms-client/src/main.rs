// main.rs (rust-am-comms-client)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
use lib::tcp::*;

// Other packages required to perform specific ASP action.
use std::collections::HashMap;
use tokio::runtime::Runtime;


fn handle_passthrough_am_req_resp_body() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <ProtocolRunRequest JSON>", args[0]);
        respond_with_failure("Invalid arguments to AM".to_string());
    }

    let json_req = &args[1];
    let req: ProtocolRunRequest = serde_json::from_str(json_req).unwrap_or_else(|error| {
        respond_with_failure(format!("Failed to parse ProtocolRunRequest: {error:?}"));
    });

    let to_plc : Plc = req.TO_PLC.clone();

    let to_sess : Attestation_Session = req.ATTESTATION_SESSION.clone();
    let to_plcmap: HashMap<String, String> = to_sess.Plc_Mapping.clone();
    let maybe_to_uuid  = to_plcmap.get(&to_plc);

    let to_uuid = match maybe_to_uuid {

        Some (val) => {val }
        _ => {""}

    };

    let to_uuid_string : String = to_uuid.to_string();

    let vreq : ProtocolRunRequest = req.clone();

    let req_str = serde_json::to_string(&vreq)?;

    let val = async {

    let stream = connect_tcp_stream(to_uuid_string, "".to_string()).await?;
    eprintln!("\nTrying to send ProtocolRunRequest: \n");
    eprintln!("{req_str}\n");

    let resp_str = am_sendRec_string(req_str,stream).await?;
    eprintln!("Got a TCP Response String: \n");
    eprintln!("{resp_str}\n");

    println!("{resp_str}");

    Ok::<(), std::io::Error> (())
    };

    let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(val) {
        Ok(x) => x,
        Err(_) => println!("Runtime failure in rust-am-client main.rs"),
    };

    std::process::exit(0);

}


fn main() -> std::io::Result<()> {
    handle_passthrough_am_req_resp_body()
}


