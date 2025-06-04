// main.rs (rust-am-comms-server)

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

// Custom package imports
use rust_am_lib::copland::*;
//use rust_am_lib::tcp::*;
use tokio::io::AsyncReadExt;

use lib::clientArgs::*;

// Other packages required to perform specific ASP action.
//use std::collections::HashMap;
use tokio::runtime::Runtime;

use tokio::net::TcpListener;
use tokio::io::AsyncWriteExt;

use std::process::{Command, Stdio};

use std::io::Write;

/*
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
*/

/*
fn handle_am_req() -> std::io::Result<()> {



}
*/



fn main() -> std::io::Result<()> {

    let args = get_am_server_args()?;
    let server_uuid = args.u_server_uuid.as_str();


    let val = async {


        let listener = TcpListener::bind(server_uuid).await?;

        loop {
            let (mut socket, _) = listener.accept().await?;

            let mut x:[u8; 4] = [0u8;4];
            let _ = socket.read_exact(&mut x).await?;

            let msg_in_size = u32::from_be_bytes(x);

            println!("message size: {}", msg_in_size); 

            //let mut y:[u8; msg_in_size] = [0u8;msg_in_size];
            let mut y = vec![0; msg_in_size.try_into().unwrap()];

            let _ = socket.read_exact(&mut y).await?;

            //let respString = String::from_utf8(&y).expect("hi");

            //println!("respString: {}", respString);
           

            let v : ProtocolRunRequest = serde_json::from_slice(&y)?;

            println!("ProtocolRunRequest: {:?}", v); 

            
            let req_str = serde_json::to_string(&v)?;



            eprintln!("\nTrying to send ProtocolRunRequest via FS: \n");
            eprintln!("{req_str}\n");

            let fs_path = args.server_am_filepath.as_str(); //"/Users/adampetz/Documents/Spring_2023/am-cakeml/build/bin/server_am";

            let manfile = args.manifest_filepath.as_str(); //"/Users/adampetz/Documents/Spring_2023/am-cakeml/am_configs/attest_remote_multinode/Manifest_P0.json";

            let aspbin_file = args.asbbin_filepath.as_str(); //"/Users/adampetz/Documents/Spring_2025/my_am_repos/asp-libs/target/release";

            let comms_file = args.commsbin_filepath.as_str(); //"/Users/adampetz/Documents/Spring_2025/my_am_repos/rust-am-clients/target/release/rust-am-comms-client-copy";


            let mut child = Command::new(fs_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .args(["-m", manfile, "-b", aspbin_file, "--comms", comms_file, "-u", server_uuid])
            .spawn()
            .expect(format!("Failed to spawn child proces: {}", fs_path).as_str());

            let mut stdin = child.stdin.take().expect("Failed to open stdin");

            std::thread::spawn(move || {
                stdin.write_all(req_str.as_bytes()).expect("Failed to write to stdin"); 
            });

            let output = child.wait_with_output().expect("Failed to read stdout");

            let err_res = output.stderr;
            let out_res : Vec<u8> = output.stdout;

            //let res = if out_res.is_empty() {err_res} 
                           // else {out_res};

            //println!("\n\n\n\n\n GOT HERE");

            let resp_str = String::from_utf8(out_res).expect("h");

            //let resp_str = am_sendRec_string_all(att_server_uuid_string, client_uuid_string, req_str)?;
            eprintln!("Got a Response String from FS in rust-am-comms-server: \n");
            eprintln!("\n\n{resp_str}\n\n");

            let resp : ProtocolRunResponse = serde_json::from_str(&resp_str)?;
            eprintln!("Decoded ProtocolRunResponse: \n");
            eprintln!("{:?}\n", resp);


            //socket.write_all(resp_str.as_bytes()).await?;
            //socket.write_all(&out_res).await?;



            let sbytes = resp_str.as_bytes();
            let sbytes_len: u32 = sbytes.len().try_into().unwrap();

            // Write a buffer of bytes representing the (u32) size of the string to be sent
            let mut wtr = vec![];
            AsyncWriteExt::write_u32(& mut wtr,sbytes_len).await?;//.unwrap();
            socket.write_all(&wtr).await?;

            // Write the string as bytes
            socket.try_write(resp_str.as_bytes())?;










           // socket.write(resp_str.as_bytes()).await?;


        }


        //println!("{resp_str}");

       Ok::<(), std::io::Error> (())

    };

    let runtime: Runtime = tokio::runtime::Runtime::new().unwrap();

    match runtime.block_on(val) {
        Ok(x) => x,
        Err(e) => println!("Runtime failure in rust-am-comms-server main.rs: {}", e),
    };

    //std::process::exit(0);




    Ok (())
    //handle_passthrough_am_req_resp_body()
}


