use clap::Parser;

const DEFAULT_TERM_PATH: &'static str = "/rust-am-clients/testing/protocols/protocol_cert_appr.json";
const DEFAULT_SERVER_UUID: &'static str = "127.0.0.1:5000";
const DEFAULT_CLIENT_UUID: &'static str = "";
const DEFAULT_PLCMAP_PATH: &'static str = "/rust-am-clients/testing/plc_maps/plcmap_default.json";
const DEFAULT_SESSION_PATH: &'static str = "/rust-am-clients/testing/attestation_sessions/session_cert_appr.json";

const DEFAULT_CVM_PATH: &'static str = "cvm";

pub const AM_REPOS_ROOT_ENV_VAR: &'static str = "AM_REPOS_ROOT";
pub const ASP_BIN_ENV_VAR: &'static str = "ASP_BIN";

fn get_local_env_var(s:String) -> std::io::Result<String> {

    let env_var_string = match std::env::var(&s) {
        Ok(val) => val,
        Err(_e) => {
            panic!("Did not set environment variable {}\n", s)
        }
    };

    Ok (env_var_string)

}

pub fn get_local_env_var_w_suffix (env_var_string:String, suffix:&str) -> std::io::Result<String> {

    let env_var_val = get_local_env_var(env_var_string)?;

    let owned_env_string = env_var_val.to_owned();
    let owned_suffix_string = suffix.to_owned();

    let together = format!("{owned_env_string}{owned_suffix_string}");

    Ok (together)
}

// Adapted from:  https://docs.rs/clap/latest/clap/
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct AspClientArgs {
    /// Path pointing to (JSON) ASP_ARGS file
    #[arg(short, long)]
    pub asp_args_filepath: String,

    /// Path pointing to ASP executable file
    #[arg(short, long)]
    pub exe_filepath: String

}

pub fn get_asp_client_args () -> std::io::Result<AspClientArgs> {
    let args: AspClientArgs = AspClientArgs::parse();
    Ok(args)
}

// Adapted from:  https://docs.rs/clap/latest/clap/
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct AmClientArgs {
    /// Path pointing to (JSON) protocol term file
    #[arg(short, long , default_value_t = 
        get_local_env_var_w_suffix(AM_REPOS_ROOT_ENV_VAR.to_string(), 
                                   DEFAULT_TERM_PATH).expect("Couldn't initialize default value for term_filepath field of AmClientArgs struct.  
                                                              Check for missing Environment Variable?"))]
    pub term_filepath: String,

    /// UUID string for AM server destination
    #[arg(short, long , default_value_t = DEFAULT_SERVER_UUID.to_string())]
    pub server_uuid: String,
    
    /// Path pointing to (JSON) Placemap file
    #[arg(short, long , default_value_t = 
        get_local_env_var_w_suffix(AM_REPOS_ROOT_ENV_VAR.to_string(), 
                                   DEFAULT_PLCMAP_PATH).expect("Couldn't initialize default value for plcmap_filepath field of AmClientArgs struct.  
                                                                Check for missing Environment Variable?"))]
    pub plcmap_filepath: String,

    /// Path pointing to (JSON) ASP GLOBAL TYPE ENVIRONMENT file
    #[arg(short, long)]
    pub env_filepath: Option<String>,

     /// Path pointing to (JSON) ASP GLOBAL ASP COMPAT MAP file
     #[arg(short, long)]
     pub glob_comps_filepath: Option<String>,

    /// Path pointing to (JSON) Attestation Session file
    #[arg(short, long , default_value_t = 
        get_local_env_var_w_suffix(AM_REPOS_ROOT_ENV_VAR.to_string(), 
                                   DEFAULT_SESSION_PATH).expect("Couldn't initialize default value for term_filepath field of AmClientArgs struct.  
                                                              Check for missing Environment Variable?"))]
    pub attestation_session_filepath: String,

    /// UUID string for AM client (default value = "" specifies OS-assigned ephemeral port)
    #[arg(short, long, default_value_t = DEFAULT_CLIENT_UUID.to_string())]
    pub client_uuid: String, 

      /// Path pointing to (JSON) attestation ASP_ARGS file
     #[arg(short, long)]
     pub b_attestation_asp_args_filepath: Option<String>,

     /// Path pointing to (JSON) appraisal ASP_ARGS file
     #[arg(short, long)]
     pub d_appraisal_asp_args_filepath: Option<String>,

     /// UUID string for appraisal AM server
     #[arg(short, long)]
     pub r_appraisal_server_uuid: Option<String>,

     /// Boolean to indicate Appraisal Summary
     #[arg(short, long, default_value_t = false)]
     pub m_appraisal_summary: bool
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
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RodeoClientArgs {
    /// Path pointing to local cvm executable
    #[arg(short, long, default_value_t = DEFAULT_CVM_PATH.to_string())]
    pub cvm_filepath: String,

    /// Path pointing to (JSON) RodeoClientRequest file
    #[arg(short, long)]
    pub req_filepath: Option<String>,
    
    /// Path pointing to (JSON) RodeoEnvironmentMap file
    #[arg(short, long)]
    pub env_filepath: Option<String>,

    /// Path pointing to (JSON) Term file
    #[arg(short, long)]
    pub term_filepath: Option<String>,

    /// Path pointing to (JSON) Attestation Session file
    #[arg(short, long)]
    pub session_filepath: Option<String>,

    /// Path for ASP_ARGS (optional)
     #[arg(short, long)]
     pub g_asp_args_filepath: Option<String>,

    /// Path for provisioned evidence (optional)
     #[arg(short, long)]
     pub provisioned_evidence_filepath: Option<String>,

    /// Boolean to indicate Appraisal Summary
    #[arg(short, long, default_value_t = false)]
    pub appraisal: bool,

    /// Path pointing to output directory for cvm requests/responses and AppraisalSummary
    #[arg(short, long)]
    pub output_dir: Option<String>,

    /// Path pointing to asp-libs bin
    #[arg(short, long, default_value_t = 
        get_local_env_var_w_suffix(ASP_BIN_ENV_VAR.to_string(), 
                                   "" ).expect("Couldn't initialize default value for libs_asp_bin field of RodeoClientArgs struct.  
                                                              Check for missing ASP_BIN Environment Variable?"))]
    pub libs_asp_bin: String,

    /// Path pointing to manifest filepath
    #[arg(short, long)]
    pub manifest_filepath: Option<String>,

    /// Path pointing to HAMR attestation directory
    #[arg(short = 'h', long, value_delimiter = ' ', num_args = 1..4)]
    pub hamr_root: Option<Vec<String>>,

}

pub fn get_rodeo_client_args () -> std::io::Result<RodeoClientArgs> {
    let args: RodeoClientArgs = RodeoClientArgs::parse();
    Ok(args)
}

// Adapted from:  https://docs.rs/clap/latest/clap/
#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct RodeoHamrClientArgs {
    /// Path pointing to HAMR attestation directory
    #[arg(short, long)]
    pub attestation_root: String,

    /// Path pointing to golden evidence file path used during provisioning and appraisal
    #[arg(short, long)]
    pub golden_evidence_filepath: String,

    /// Path pointing to output file path for the generated Copland protocol term
    #[arg(short, long)]
    pub output_term_filepath: String
}

pub fn get_rodeo_hamr_client_args () -> std::io::Result<RodeoHamrClientArgs> {
    let args: RodeoHamrClientArgs = RodeoHamrClientArgs::parse();
    Ok(args)
}