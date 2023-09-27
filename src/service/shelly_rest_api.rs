use std::error::Error;
use std::fmt::{Display, format, Formatter};
use std::fs::read_to_string;
use std::io::{BufRead, Read};
use digest_auth::AuthContext;
use log::{debug, error, info};
use reqwest::blocking::{RequestBuilder, Response};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use diqwest::blocking::WithDigestAuth;

pub fn save_script_to_shelly(file_path: &str) -> Result<(), Box<dyn Error>>{
    let mut id: Option<i32> = None;

    let file: String = String::new();
    let file_content = read_to_string(file_path)?;

    let shelly = Shelly::new()?;
    let available_scripts = shelly.list()?;
    debug!("{:?}", available_scripts);

    shelly.put_code(1, format!("{}", read_to_string(file_path)?), false)?;

    info!("The file has been correctly uploaded !");

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
struct Script {
    id: i32,
    name: String,
    enable: bool,
    running: bool,
}

#[derive(Serialize, Deserialize)]
struct Chunk {
    id: i32,
    code: String,
    append: bool,
}

struct Shelly {
    client: reqwest::blocking::Client,
    host: String,
    username: String,
    password: String,
}

impl Shelly {
    pub fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Shelly {
                client: reqwest::blocking::Client::new(),
                host: std::env::var("shelly-host")?,
                username: "admin".to_string(),
                password: std::env::var("shelly-password")?,
            })
    }

    fn get_url(&self, uri: &str) -> String {
        format!("http://{}{uri}", self.host)
    }

    fn put_code(&self, id: i32, data: String, append: bool) -> Result<(), Box<dyn Error>>{
        let uri = "/rpc/Script.PutCode";
        let url = self.get_url(uri);
        debug!("{}", url);

        let chunk = Chunk {
            id: id,
            code: data,
            append: append,
        };
        let json = serde_json::to_string(&chunk)?;

        let response = self.client
            .post(&url)
            .header("Content-Length", HeaderValue::from(json.len()))
            .body(json)
            .send_with_digest_auth(&self.username, &self.password)?;


        if response.status().is_client_error() {
            return Err(Box::new(ClientRequestError { code: response.status().as_u16() }));
        }
        else if response.status().is_server_error() {
            return Err(Box::new(InternalServerError { code: response.status().as_u16() }));
        }

        Ok(())
    }

    fn list(&self) -> Result<Vec<Script>, Box<dyn Error>> {
        let uri = "/rpc/Script.List";
        let url = self.get_url(uri);

        let response = self.client
            .get(&url)
            .send_with_digest_auth(&self.username, &self.password)?;

        if response.status().is_client_error() {
            return Err(Box::new(ClientRequestError { code: response.status().as_u16() }));
        }
        else if response.status().is_server_error() {
            return Err(Box::new(InternalServerError { code: response.status().as_u16() }));
        }

        let body = response.text()?;
        let data: serde_json::Value = serde_json::from_str(&body)?;
        if let Some(simple_vec) = data["scripts"].as_array() {
            let scripts: Vec<Script> = simple_vec
                .iter()
                .filter_map(|script| serde_json::from_value(script.clone()).ok())
                .collect();
            return Ok(scripts);
        }

        Err(Box::new(ParseScriptsInfo))
    }
}





// Error thing
#[derive(Debug)]
struct ParseError;
impl Error for ParseError {}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to parse the id of the file, please provide it in the first line of the file. E.G '//3'")
    }
}

#[derive(Debug)]
struct InternalServerError {
    code: u16,
}
impl Error for InternalServerError {}
impl Display for InternalServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = format!("There is something wrong going on with the shelly -> response code {}", self.code);
        write!(f, "{msg}")
    }
}

#[derive(Debug)]
struct ClientRequestError {
    code: u16,
}
impl Error for ClientRequestError {}
impl Display for ClientRequestError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let msg = format!("Something goes wrong when trying to do the request to the shelly -> response code {}", self.code);
        write!(f, "{msg}")
    }
}

#[derive(Debug)]
struct ParseScriptsInfo;
impl Error for ParseScriptsInfo {}
impl Display for ParseScriptsInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unable to parse correctly the list of scripts from the Shelly")
    }
}


