use std::thread;
use log::{error, info};
use crate::file_checker;
use crate::logger::Logger;

///When called will start logging from the websocket
///
/// * `path` - the path of the directory that will be check for update
/// * `ws_port` - the websocket port
/// * `autorun` - the autorun parameter
pub fn debug(path: &str, ws_port: i32, autorun: bool) {

    std::env::set_var("shelly-port", ws_port.to_string());
    std::env::set_var("shelly-autorun", autorun.to_string());

    info!("Path : {}", path);
    info!("WS Port : {}", ws_port);
    info!("Autorun : {}", autorun);

    ///[thread::spawn()] create a new thread and move it, like this it can be independent
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