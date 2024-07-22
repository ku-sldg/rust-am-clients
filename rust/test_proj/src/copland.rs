

pub mod copland {

use std::collections::HashMap;
use bytestring;


use serde::{Deserialize, Serialize};
use serde_json::Result;

type Plc = String;
type N_ID = String;
type ASP_ID = String;
type TARG_ID = String;
type ASP_ARGS = HashMap<String, String>;


#[derive(Serialize, Deserialize)]
struct ASP_PARAMS {
    ASP_ID: ASP_ID,
    ASP_ARGS: ASP_ARGS,
    ASP_PLC: Plc,
    ASP_TARG_ID: TARG_ID
}

#[derive(Serialize, Deserialize)]
enum FWD {
    COMP,
    ENCR,
    EXTD(u32),
    KILL,
    KEEP
}

enum Evidence {
    mt,
    nn(N_ID),
    uu(Plc, FWD, ASP_PARAMS, Box<Evidence>),
    ss(Box<Evidence>, Box<Evidence>)
}

#[derive(Serialize, Deserialize)]
enum SP {
    ALL,
    NONE
}

#[derive(Serialize, Deserialize)]
#[serde(tag = "ASP_CONSTRUCTOR", content = "TERM_BODY")]
pub enum ASP {
    NULL,
    CPY,
    ASPC(SP, FWD, ASP_PARAMS),
    SIG,
    HSH,
    ENC(Plc)
}

type Split = (SP, SP);


#[derive(Serialize, Deserialize)]
#[serde(tag = "TERM_CONSTRUCTOR", content = "TERM_BODY")]
pub enum Term {
    asp(ASP),
    att(Plc, Box<Term>),
    lseq(Box<Term>, Box<Term>),
    bseq(Split, Box<Term>, Box<Term>),
    bpar(Split, Box<Term>, Box<Term>)
}

type BS = bytestring::ByteString;

type RawEvT = Vec<String>;  //Vec<BS>;

#[derive(Serialize, Deserialize, Debug)]
//#[serde(untagged)]
//#[serde(tag = "RawEv", content = "TERM_BODY")]
pub enum RawEv {
    RawEv(RawEvT)
}

#[derive(Serialize, Deserialize)]
pub struct ProtocolRunRequest {
    pub TYPE:  String,
    pub ACTION:  String,
    pub REQ_PLC:  Plc,
    pub TERM:  Term, 
    pub RAWEV:  RawEv
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolRunResponse {
    TYPE:  String,
    ACTION:  String,
    SUCCESS:  bool,
    PAYLOAD:  RawEv
}

}

