mod file_checker;
mod service;
mod logger;

use std::thread;
use clap::Parser;
use log::{error, info};
use crate::logger::Logger;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///The directory where the utilitary will check for edited file
    #[arg(long, default_value_t = String::from("./"))]
    path: String,

    ///Run the script when is uploaded to the Shelly
    #[arg(long, default_value_t = true)]
    autorun: bool,

    ///The level of log, often used when developing or debugging the utilitary ('info', 'error', 'debug', 'all')
    #[arg(long, default_value_t = String::from("info"))]
    log: String,

    ///The IP of the host
    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

    ///The port that the websocket will use to get the logs on the Shelly (generaly you don't have to edit this one)
    #[arg(long, default_value_t = 80)]
    ws_port: i32,

    ///The username of the account on the Shelly
    #[arg(long, default_value_t = String::from("admin"))]
    username: String,

    ///The password used to connect to account on the Shelly
    #[arg(long, default_value_t = String::from("123456"))]
    password: String,
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
    std::env::set_var("shelly-port", &args.ws_port.to_string());
    std::env::set_var("shelly-username", &args.username);
    std::env::set_var("shelly-password", &args.password);
    std::env::set_var("shelly-autorun", &args.autorun.to_string());

    env_logger::init();

    info!("Path : {}", &args.path);
    info!("Shelly host ip: {}", &args.host);
    info!("Shelly password: {}", &args.password);
    info!("Shelly Remote Helper have correctly started !");

    thread::spawn(move || {
        let logger = Logger::new();
        if logger.is_ok() {
            let error = logger.unwrap().start();
            error!("Something goes wrong and kill the logger, please restart the app -> {}", error.unwrap_err());
        } else {
            error!("Failed to start the logger -> {}", logger.unwrap_err());
        }
    });
    file_checker::FileChecker::new().start(&args.path);
}
