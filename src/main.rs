mod file_checker;
mod service;

use clap::Parser;
use log::info;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value_t = String::from("./"))]
    path: String,

    #[arg(long, default_value_t = String::from("info"))]
    log: String,

    #[arg(long, default_value_t = String::from("127.0.0.1"))]
    host: String,

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

    std::env::set_var("RUST_LOG", "shelly_remote_helper");
    std::env::set_var("shelly-host", &args.host);
    std::env::set_var("shelly-password", &args.password);
    env_logger::init();

    info!("Path to check: {}", &args.path);
    info!("Shelly host ip: {}", &args.host);
    info!("Shelly password: {}", &args.password);
    info!("Shelly Remote Helper have correctly started !");

    file_checker::FileChecker::new().start_checking(&args.path)
}
