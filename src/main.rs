mod file_checker;
mod service;
mod logger;
mod debugger;
mod action;

use clap::{Parser, Subcommand};
use log::info;

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
    #[arg(long, default_value_t = String::from("admin"))]
    username: String,

    ///The password used to connect to account on the Shelly
    #[arg(long, required = true)]
    password: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ///Setup configuration file for your loved IDE
    Setup {
        ///Will create the config for the Visual Studio Code editor "./.vscode/tasks.json" (Will create the file in the current directory)
        #[arg(long)]
        vs_code: bool,
    },

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

    ///Run script in the Shelly by the name
    Start {
        script_name: String,
    },

    ///Stop script in the Shelly by the name
    Stop {
        script_name: String,
    },

    ///Show the available script on the Shelly
    List {}
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
        } => debugger::debug(&path, ws_port, autorun),
        Commands::Start {
            script_name
        } => action::start(&script_name),
        Commands::Stop {
            script_name
        } => action::stop(&script_name),
        Commands::Setup {
            vs_code,
        } => action::setup(vs_code),
        Commands::List {} => action::list(),
    }
}



