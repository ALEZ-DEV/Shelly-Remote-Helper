use std::error::Error;
use std::thread;
use std::time::Duration;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use tungstenite::connect;
use url::Url;

///The Logger data
#[derive(Debug)]
pub struct Logger {
    ///The IP of the Shelly
    host: String,
    ///The websocket port
    port: i32,
}

///Representation of a message from the websocket
#[derive(Serialize, Deserialize)]
struct LogMessage {
    ts: f64,
    level: i64,
    data: String,
}

impl Logger {
    ///Create a new instance of the Logger
    pub fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Self {
            host: std::env::var("shelly-host")?,
            port: std::env::var("shelly-port")?.parse::<i32>()?,
        })
    }

    ///Start the logger and will try to connect to the shelly
    pub fn start(&self) -> Result<(), Box<dyn Error>>{
        let url = format!("ws://{}:{}/debug/log", self.host, self.port);
        let mut retry_time = 0;

        loop {
            thread::sleep(Duration::from_secs(retry_time));

            let connection = connect(Url::parse(&url)?);

            if connection.is_err() {
                retry_time += 5;
                error!("Failed to listen the websocket at {}, retrying in {}s ...", self.host, retry_time);
                let error = connection.unwrap_err();
                error!("Due to -> {}", error);
                continue;
            }

            let (mut socket, _) = connection?;

            loop {
                let read_msg = socket.read();
                if read_msg.is_err() {
                    error!("Failed to read the message");
                }

                let msg = read_msg.unwrap();
                let text = msg.to_text()?;
                let log_msg = serde_json::from_str::<LogMessage>(text);

                if log_msg.is_err() {
                    debug!("Failed to deserialize the message");
                    debug!("raw message -> {}", text)
                } else {
                    let info_log_msg = log_msg?;

                    match info_log_msg.level {
                        -1 => info!("{}", info_log_msg.data),
                        _ => debug!("{}", info_log_msg.data),
                    }
                }
            }
        }
    }
}