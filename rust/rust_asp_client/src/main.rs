#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

use std::collections::HashMap;
use std::process::Command;
use std::io::{self, Write};

use rust_am_lib::copland::*;
use rust_am_lib::copland::RawEv::*;

fn main() -> std::io::Result<()> {

    let aid : ASP_ID = "hi".to_string();
    let args : ASP_ARGS = HashMap::from([]);

    let aspreq: ASPRunRequest = 
        ASPRunRequest {
            TYPE: "REQUEST".to_string(),
            ACTION: "ASP_RUN".to_string(),
            ASP_ID: aid,
            ASP_ARGS: args,
            ASP_PLC: "P0".to_string(),
            ASP_TARG_ID: "TARG".to_string(),
            RAWEV: RawEv(vec![])
        };

    let req_str = serde_json::to_string(&aspreq)?;

    let _ = println!("{req_str}");

    let prog_str = 
            "/Users/adampetz/Documents/Summer_2024/asp-libs/attestation_asps/rust_asps/target/debug/r_uptime_id".to_string();
    //"echo".to_string();
    // prog_str.push_str(&req_str);

    let args_str = req_str;
    //args_str.push_str(" | echo");
    //let args_str = ""; //"hihihihgi";

    let output = Command::new(prog_str)
            .arg(args_str)
            //.spawn()
            .output()
            .expect("failed to execute command");

    io::stdout().write_all(&output.stdout).unwrap();



        Ok (())


    /*
    use std::process:Command;

    fn main(){
    Command::new("ls")
            .arg("-a")
            .spawn()
            .expect("failed to execute command");
    }

    */
        



}
