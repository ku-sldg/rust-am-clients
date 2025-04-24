use clap::Parser;

const DEFAULT_TERM_PATH: &'static str = "/testing/protocol_cert_appr.json";
const DEFAULT_SERVER_UUID: &'static str = "127.0.0.1:5000";
const DEFAULT_PLCMAP_PATH: &'static str = "/testing/plcmap_default.json";
//const DEFAULT_TYPE_ENV_PATH: &'static str = "/testing/glob_type_env_default.json";
//const DEFAULT_GLOB_COMPS_PATH: &'static str = "/testing/glob_comps_default.json";
const DEFAULT_SESSION_PATH: &'static str = "/testing/session_cert_appr.json";

const DEFAULT_ENV_PATH: &'static str = "/testing/env_resolute_cert_appr.json";

const AM_CLIENTS_ENV_VAR: &'static str = "AM_CLIENTS_ROOT";

fn get_local_env_var(s:String) -> std::io::Result<String> {

    let env_var_string = match std::env::var(&s) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable {}\n", s)
        }
    };

    Ok (env_var_string)

}

fn get_local_env_var_w_suffix (env_var_string:String, suffix:&str) -> std::io::Result<String> {

    let env_var_val = get_local_env_var(env_var_string)?;

    let owned_env_string = env_var_val.to_owned();
    let owned_suffix_string = suffix.to_owned();

    let together = format!("{owned_env_string}{owned_suffix_string}");

    Ok (together)
}

// Adapted from:  https://docs.rs/clap/latest/clap/
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct AmClientArgs {
    /// Path pointing to (JSON) protocol term file
    #[arg(short, long , default_value_t = 
        get_local_env_var_w_suffix(AM_CLIENTS_ENV_VAR.to_string(), 
                                   DEFAULT_TERM_PATH).expect("Couldn't initialize default value for term_filepath field of AmClientArgs struct.  
                                                              Check for missing Environment Variable?"))]
    pub term_filepath: String,

    /// UUID string for AM server destination
    #[arg(short, long , default_value_t = DEFAULT_SERVER_UUID.to_string())]
    pub server_uuid: String,
    
    /// Path pointing to (JSON) Placemap file
    #[arg(short, long , default_value_t = 
        get_local_env_var_w_suffix(AM_CLIENTS_ENV_VAR.to_string(), 
                                   DEFAULT_PLCMAP_PATH).expect("Couldn't initialize default value for plcmap_filepath field of AmClientArgs struct.  
                                                                Check for missing Environment Variable?"))]
    pub plcmap_filepath: String,

    /// Path pointing to (JSON) ASP GLOBAL TYPE ENVIRONMENT file
    #[arg(short, long /*, default_value_t = Some (
        get_local_env_var_w_suffix(AM_CLIENTS_ENV_VAR.to_string(), 
                                   DEFAULT_TYPE_ENV_PATH).expect("Couldn't initialize default value for env_filepath field of AmClientArgs struct.  
                                                                  Check for missing Environment Variable?"))*/)]
    pub env_filepath: Option<String>,

     /// Path pointing to (JSON) ASP GLOBAL ASP COMPAT MAP file
     #[arg(short, long /*, default_value_t =
        get_local_env_var_w_suffix(AM_CLIENTS_ENV_VAR.to_string(), 
                                   DEFAULT_GLOB_COMPS_PATH).expect("Couldn't initialize default value for glob_comps_filepath field of AmClientArgs struct.  
                                                                    Check for missing Environment Variable?")*/)]
     pub glob_comps_filepath: Option<String>,

    /// Path pointing to (JSON) Attestation Session file
    #[arg(short, long , default_value_t = 
        get_local_env_var_w_suffix(AM_CLIENTS_ENV_VAR.to_string(), 
                                   DEFAULT_SESSION_PATH).expect("Couldn't initialize default value for term_filepath field of AmClientArgs struct.  
                                                              Check for missing Environment Variable?"))]
    pub attestation_session_filepath: String,
}

fn validate_am_client_args (args:&AmClientArgs) -> () {

    let maybe_env_filepath = args.env_filepath.clone(); 
    let maybe_glob_comps_filepath = args.glob_comps_filepath.clone();
    let warning_message: String = 
        "NOTE:  One of env_filepath or glob_comps_filepath NOT provided as an arg.  Using the Attestation Session provided (or the DEFAULT if no session provided...)! ".to_string();

    match (maybe_env_filepath, maybe_glob_comps_filepath) {
        (Some (_), Some (_)) => {println!("NOTE:  Overriding Attestation Session with provided env_filepath and glob_comps_filepath args");}
        _ => {println!("{warning_message}");}
    }
}

pub fn get_am_client_args () -> std::io::Result<AmClientArgs> {
    let args: AmClientArgs = AmClientArgs::parse();
    validate_am_client_args(&args);
    Ok(args)
}

// Adapted from:  https://docs.rs/clap/latest/clap/
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct ResoluteClientArgs {
    /// Path pointing to (JSON) ResoluteClientRequest file
    #[arg(short, long)]
    pub req_filepath: String,

    /// UUID string for AM server destination
    #[arg(short, long , default_value_t = DEFAULT_SERVER_UUID.to_string())]
    pub server_uuid: String,
    
    /// Path pointing to (JSON) ResoluteEnvironmentMap file
    #[arg(short, long , default_value_t = 
        get_local_env_var_w_suffix(AM_CLIENTS_ENV_VAR.to_string(), 
                                   DEFAULT_ENV_PATH).expect("Couldn't initialize default value for env_filepath field of ResoluteClientArgs struct.  
                                                                Check for missing Environment Variable?"))]
    pub env_filepath: String

}

pub fn get_resolute_client_args () -> std::io::Result<ResoluteClientArgs> {
    let args: ResoluteClientArgs = ResoluteClientArgs::parse();
    Ok(args)
}