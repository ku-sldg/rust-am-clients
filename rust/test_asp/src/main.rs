use std::collections::HashMap;
use std::process::Command;


use test_proj::copland::*;

use test_proj::copland::SP::*;
use test_proj::copland::RawEv::*;
use test_proj::copland::ASPRunRequest;

use test_proj::tcp::*;
use test_proj::json::*;

//use test_proj::json::encode_gen;

fn main() {
    let v : SP = NONE;
    //println!("Hello, world!");

    let aid : ASP_ID = "hi".to_string();
    let args : ASP_ARGS = HashMap::from([]);

    let aspreq: ASPRunRequest = 
        ASPRunRequest {
            TYPE: "REQUEST".to_string(),
            ACTION: "ASP_RUN".to_string(),
            ASP_ID: aid,
            ASP_ARGS: args,
            ASP_PLC: "P0".to_string(),
            ASP_TARGID: "TARG".to_string(),
            RAWEV: RawEv(vec![])
        };

    let req_str = encode_gen(&aspreq);

    Command::new("ls")
            .arg("-a")
            .spawn()
            .expect("failed to execute command");


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
