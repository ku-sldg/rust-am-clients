

pub mod copland {

use std::collections::HashMap;
use bytestring;


use serde::{Deserialize, Serialize, Serializer};


pub type Plc = String;
type N_ID = String;
type ASP_ID = String;
type TARG_ID = String;
type ASP_ARGS = HashMap<String, String>;


#[derive(Serialize, Deserialize, Debug)]
pub struct ASP_PARAMS {
    pub ASP_ID: ASP_ID,
    pub ASP_ARGS: ASP_ARGS,
    pub ASP_PLC: Plc,
    pub ASP_TARG_ID: TARG_ID
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "FWD_CONSTRUCTOR", content = "FWD_BODY")]
pub enum FWD {
    COMP,
    ENCR,
    EXTD(String),
    KILL,
    KEEP
}

enum Evidence {
    mt,
    nn(N_ID),
    uu(Plc, FWD, ASP_PARAMS, Box<Evidence>),
    ss(Box<Evidence>, Box<Evidence>)
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SP {
    ALL,
    NONE
}

#[derive(Serialize, Deserialize, Debug)]
//#[derive(Debug)]
#[serde(tag = "ASP_CONSTRUCTOR", content = "ASP_BODY")]
pub enum ASP {
    NULL,
    CPY,
    ASPC(SP, FWD, ASP_PARAMS),
    SIG,
    HSH,
    ENC(Plc)
}


/*
impl Serialize for ASP {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
            match *self {
                ASP::NULL => serializer.serialize_unit_variant("ASP", 0, "NULL"),
                ASP::CPY =>  serializer.serialize_unit_variant("ASP", 1, "CPY"),
                _ => serializer.serialize_unit_variant("ASP", 0, "NULL")
            }
     }
    }


impl Deserialize for ASP {

}
*/

type Split = (SP, SP);


#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "TERM_CONSTRUCTOR", content = "TERM_BODY")]
pub enum Term {
    asp(ASP),
    att(Plc, Box<Term>),
    lseq(Box<Term>, Box<Term>),
    bseq(Split, Box<Term>, Box<Term>),
    bpar(Split, Box<Term>, Box<Term>)
}

//type BS = bytestring::ByteString;

type RawEvT = Vec<String>;  //Vec<BS>;

#[derive(Serialize, Deserialize, Debug)]
//#[serde(untagged)]
//#[serde(tag = "RawEv_CONSTRUCTOR", content = "RawEv_BODY")]
pub enum RawEv {
    RawEv(RawEvT)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Attestation_Session {
    pub Session_Plc:  Plc,
    pub Plc_Mapping:  HashMap<Plc, String>,
    pub PubKey_Mapping:  HashMap<Plc, String>
}



#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolRunRequest {
    pub TYPE:  String,
    pub ACTION:  String,
    pub REQ_PLC:  Plc,
    pub TERM:  Term, 
    pub RAWEV:  RawEv,
    pub ATTESTATION_SESSION: Attestation_Session
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProtocolRunResponse {
    TYPE:  String,
    ACTION:  String,
    SUCCESS:  bool,
    PAYLOAD:  RawEv
}

}

