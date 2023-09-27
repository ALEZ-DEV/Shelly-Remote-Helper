use std::error::Error;
use std::thread;
use std::time::Duration;
use log::{error, info};
use serde::{Deserialize, Serialize};
use tungstenite::connect;
use url::Url;

#[derive(Debug)]
pub struct Logger {
    host: String,
    port: i32,
}

#[derive(Serialize, Deserialize)]
struct LogMessage {
    ts: f64,
    level: i64,
    data: String,
}
impl Logger {
    pub fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Self {
            host: std::env::var("shelly-host")?,
            port: std::env::var("shelly-port")?.parse::<i32>()?,
        })
    }

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
                let msg = socket.read();

                if msg.is_err() {
                    error!("Failed to read the message");
                } else {
                    let log_msg: LogMessage = serde_json::from_str(msg.unwrap().to_text()?)?;

                    info!("level: {} data: {}",log_msg.level, log_msg.data);
                }
            }
        }
    }
}