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
    log: String
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
    env_logger::init();

    info!("path: {}", args.path);
    info!("Shelly Remote Help have correctly started !");

    file_checker::FileChecker::new().start_checking("C:/Users/AGonzalez/Documents")
}
