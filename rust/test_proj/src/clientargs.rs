#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

const HELP: &str = "\
App

USAGE:
  app [OPTIONS]

FLAGS:
  -h, --help            Prints help information

OPTIONS:
  --termfile f        f = JSON filepath with Copland term (i.e. 'myterm.json').  Takes precedence over --term option.
  --term s            s = ['cert' | 'filehash' | 'bg' | 'parmut'] (default: 'cert')
  --attserver uuid    uuid = 'host:port' of attestation server (default: 'localhost:5000')
  --appserver uuid    uuid = 'host:port' of attestation server (default: 'localhost:5003')

";
#[derive(Clone)]
pub struct ClientArgsOpt {
    opt_term_name: Option<String>,
    opt_term_filename: Option<String>,
    opt_att_server_uuid: Option<String>,
    opt_app_server_uuid: Option<String>,
}

struct _ClientArgs {
    term_name: String,
    term_filename: String,
    att_server_uuid: String,
    app_server_uuid: String,
}

pub fn get_client_args()-> std::io::Result<ClientArgsOpt> {
    match parse_args() {
        Err (_e) => panic!("hi"),
        Ok (optArgs) => 
        {
            Ok (optArgs)
        }
    }
}

/* Inspired by:  https://github.com/RazrFalcon/pico-args/blob/master/examples/app.rs */
fn parse_args() -> Result<ClientArgsOpt, pico_args::Error> {
    let mut pargs = pico_args::Arguments::from_env();

    // Help has a higher priority and should be handled separately.
    if pargs.contains(["-h", "--help"]) {
        print!("{}", HELP);
        std::process::exit(0);
    }

    let args = ClientArgsOpt {
        // Parses an optional value that implements `FromStr`.
        opt_term_name: pargs.opt_value_from_str("--term")?,
        // Parses an optional value that implements `FromStr`.
        opt_term_filename: pargs.opt_value_from_str("--termfile")?,
        // Parses an optional value that implements `FromStr`.
        opt_att_server_uuid: pargs.opt_value_from_str("--attserver")?,
        opt_app_server_uuid: pargs.opt_value_from_str("--appserver")?,
    };

    // It's up to the caller what to do with the remaining arguments.
    let remaining = pargs.finish();
    if !remaining.is_empty() {
        eprintln!("Warning: unused arguments left: {:?}.", remaining);
    }

    Ok(args)
}

pub fn get_term_filepath (argsin:ClientArgsOpt) -> String {

    const FILEHASH_FILEPATH : &str = "filehash.json";
    const CERT_FILEPATH : &str = "cert.json";
    const BG_FILEPATH : &str = "bg.json";
    const PARMUT_FILEPATH : &str = "parmut.json";

    const FILEHASH : &str = "filehash";
    const CERT : &str = "cert";
    const BG : &str = "bg";
    const PARMUT : &str = "parmut";

        match argsin {
            ClientArgsOpt {
                opt_term_name: maybe_term,
                opt_term_filename: maybe_filename,
                ..
            } => 

            match maybe_filename {
                Some (s) => s,
                None => 
                    match maybe_term {
                        Some (tn) =>
                            match &tn[..] {
                                CERT => CERT_FILEPATH.to_string(),
                                FILEHASH => FILEHASH_FILEPATH.to_string(),
                                BG => BG_FILEPATH.to_string(),
                                PARMUT => PARMUT_FILEPATH.to_string(),
                                _ => {panic!("Error:  not one of pre-defined client demo phrases")}
                            }
                            None => 
                            {
                                println!("Warning:  No --term or --termfile options provided.  Using 'cert' (with file 'cert.json') example as default.");
                                CERT_FILEPATH.to_string()
                            }
                    }
            }
        }
}

pub fn get_att_uuid (argsin:ClientArgsOpt) -> String {

    const att_uuid_default : &str = "localhost:5000";

        match argsin {
            ClientArgsOpt {
                opt_att_server_uuid: maybe_att_server_uuid,
                ..
            } => 
                match maybe_att_server_uuid {
                    Some (s) => s,
                    None => 
                    {
                        println!("Warning:  No --attserver option provided.  Using 'localhost:5000' as default.");
                        att_uuid_default.to_string()
                    }  
                }
        }
}

pub fn get_app_uuid (argsin:ClientArgsOpt) -> String {

    const app_uuid_default : &str = "localhost:5003";

        match argsin {
            ClientArgsOpt {
                opt_app_server_uuid: maybe_app_server_uuid,
                ..
            } => 
                match maybe_app_server_uuid {
                    Some (s) => s,
                    None => 
                    {
                        println!("Warning:  No --appserver option provided.  Using 'localhost:5003' as default.");
                        app_uuid_default.to_string()
                    }  
                }
        }
}

/*
fn validate_args(argsin:ClientArgsOpt) -> ClientArgs {

    match argsin {
        ClientArgsOpt {
            opt_term_name: maybe_term,
            opt_term_filename: maybe_filename, 
            opt_att_server_uuid: maybe_att_server_uuid, 
            opt_app_server_uuid: maybe_app_server_uuid
        } => 
            {
                let termname = match maybe_term {
                    Some (s) => s,
                    None => "cert".to_string(), /* Default to the cert phrase */
                };

                let term_filename : String = "".to_string();

                let att_uuid = "".to_string();
                let app_uuid = "".to_string();

                
                ClientArgs {
                    term_name: termname, 
                    term_filename: term_filename,
                    att_server_uuid: att_uuid, 
                    app_server_uuid: app_uuid,
                }
            }

    }


}
*/