mod file_checker;
mod service;
mod logger;

use std::thread;
use clap::{Parser, Subcommand};
use log::{error, info};
use crate::logger::Logger;
use crate::service::shelly_rest_api::Shelly;
use crate::service::vscode_tasks::SetupVsCode;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,

    ///The level of log, often used when developing or debugging the utilitary ('info', 'error', 'debug', 'all')
    #[arg(long, default_value_t = String::from("info"))]
    log: String,

    ///The IP of the host
    #[arg(long, required = true)]
    host: String,

    ///The username of the account on the Shelly
    #[arg(long, default_value_t = String::from("info"))]
    username: String,

    ///The password used to connect to account on the Shelly
    #[arg(long, required = true)]
    password: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ///Start the Shelly debugger
    Debug {
        ///The directory where the utilitary will check for edited file
        #[arg(long, default_value_t = String::from("./"))]
        path: String,

        ///The port that the websocket will use to get the logs on the Shelly (generaly you don't have to edit this one)
        #[arg(long, default_value_t = 80)]
        ws_port: i32,

        ///If indicated, directly run the script when is uploaded to the Shelly
        #[arg(short, long)]
        autorun: bool,
    },

    ///Run script in the Shelly by the name, if empty, will not be used
    Start {
        script_name: String,
    },

    ///Setup configuration file for your loved IDE
    Setup {
        ///Will create the config for the Visual Studio Code editor "./.vscode/tasks.json" (Will create the file in the current directory)
        #[arg(long)]
        vs_code: bool,
    }
}

fn main() {
    let args = Args::parse();

    match args.log.to_lowercase().as_str() {
        "info" => std::env::set_var("RUST_LOG", "info"),
        "error" => std::env::set_var("RUST_LOG", "error"),
        "debug" => std::env::set_var("RUST_LOG", "debug"),
        "all" => std::env::set_var("RUST_LOG", "shelly_remote_helper"),
        other => {
            eprintln!("You can only specify 'info', 'error' or 'debug' into log, but {} is not a valid value", other);
            return;
        }
    }

    std::env::set_var("shelly-host", &args.host);
    std::env::set_var("shelly-username", &args.username);
    std::env::set_var("shelly-password", &args.password);

    env_logger::init();

    info!("Shelly host ip: {}", &args.host);
    info!("Shelly password: {}", &args.password);
    info!("Shelly Remote Helper have correctly started !");


    match args.command {
        Commands::Debug {
            path,
            ws_port,
            autorun,
        } => debug(&path, ws_port, autorun),
        Commands::Start {
            script_name: file_name,
        } => start(&file_name),
        Commands::Setup {
            vs_code,
        } => setup(vs_code),
    }
}

fn setup(vscode: bool) {
    if vscode {
        let result = SetupVsCode::new();

        if result.is_err() {
            error!("Failed to create the config on initialisation");
            error!("Due to -> {}", result.unwrap_err());
            return;
        }

        let setup_vs_code= result.as_ref().unwrap();

        let write_result = setup_vs_code.write();

        if write_result.is_err() {
            error!("Failed to write the file");
            error!("Due to -> {}", result.unwrap_err());
            return;
        } else {
            info!("Config file created, you can now close this console");
        }
    } else {
        error!("You have to choose a config profile to create the config !")
    }
}

fn debug(path: &str, ws_port: i32, autorun: bool) {

    std::env::set_var("shelly-port", ws_port.to_string());
    std::env::set_var("shelly-autorun", autorun.to_string());

    info!("Path : {}", path);
    info!("WS Port : {}", ws_port);
    info!("Autorun : {}", autorun);

    thread::spawn(move || {
        let logger = Logger::new();
        if logger.is_ok() {
            let error = logger.unwrap().start();
            error!("Something goes wrong and kill the logger, please restart the app -> {}", error.unwrap_err());
        } else {
            error!("Failed to start the logger -> {}", logger.unwrap_err());
        }
    });
    file_checker::FileChecker::new().start(path);
}

fn start(script_name: &str) {

    std::env::set_var("shelly-autorun", true.to_string());

    let result = Shelly::new();
    if result.is_err() {
        error!("oh my... how it can be wrong ? go check main.rs line 96 to 99");
        return;
    }

    let shelly = result.unwrap();

    let list_result = shelly.script_list();
    if list_result.is_err() {
        error!("Failed to get the script list from the Shelly");
        return;
    }

    let script_list = list_result.unwrap();
    let script = script_list
        .iter()
        .filter_map(|script| {
            if script.name == script_name {
                Some(script)
            } else {
                None
            }
        }).next();

    if script.is_none() {
        error!("can't start the script, because is not existent");
        return;
    }

    let start_result = shelly.script_start(script.unwrap());

    if start_result.is_ok() {
        info!("script started !");
    } else {
        error!("Unable to start script");
        error!("Due to -> {}", start_result.unwrap_err())
    }
}