use clap::Parser;


const DEFAULT_SERVER_UUID: &'static str = "localhost:5000";

// Adapted from:  https://docs.rs/clap/latest/clap/
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct AmClientArgs {
    /// Path pointing to (JSON) protocol term file
    #[arg(short, long)]
    pub term_filepath: String,

    /// UUID string for AM server destination
    #[arg(short, long , default_value_t = DEFAULT_SERVER_UUID.to_string())]
    pub server_uuid: String

}

#[allow(non_snake_case)]
pub fn get_am_client_args () -> std::io::Result<AmClientArgs> {
    let args: AmClientArgs = AmClientArgs::parse();
    Ok(args)
}