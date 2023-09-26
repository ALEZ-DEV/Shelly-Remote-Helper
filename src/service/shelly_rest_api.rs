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
    let mut append = false;
    let mut id: Option<i32> = None;

    let mut file: String = String::new();

    for line in read_to_string(file_path)?.lines() {
        if append == false && id == None {
            match line.replace("//", "").parse::<i32>() {
                Err(_) => {
                    //error!("failed to parse the id of the file, please provide it in the first line of the file. E.G '//3'");
                    return Err(Box::new(ParseError));
                }
                Ok(number) => {
                    id = Some(number);
                    debug!("Starting uploading the script {} to the script id {}", file_path, id.unwrap());
                }
            }
        }

        append = true;
    }

    put_chunck(id.ok_or("")?, format!("{}", read_to_string(file_path)?), false)?;

    info!("The file has been correctly uploaded !");

    Ok(())
}

#[derive(Serialize, Deserialize)]
struct Chunk {
    id: i32,
    code: String,
    append: bool,
    auth: Option<Authorization>,
}
#[derive(Serialize, Deserialize)]
struct Authorization {
    realm: String,
    username: String,
    nonce: String, //u64
    cnonce: String, //u64
    response: String,
    algorithm: String,
}

#[derive(Serialize, Deserialize)]
struct ShellyResponse {
    id: i32,
    src: String,
    dst: String,
    error: Option<ShellyErrorResponse>,
}
#[derive(Serialize, Deserialize)]
struct ShellyErrorResponse {
    code: i32,
    message: String,
}

fn put_chunck(id: i32, data: String, append: bool) -> Result<(), Box<dyn Error>>{
    let host = std::env::var("shelly-host")?;
    let uri = "/rpc/Script.PutCode";
    let url = format!("http://{host}{uri}");
    debug!("{}", url);

    let chunk = Chunk {
        id: id,
        code: data,
        append: append,
        auth: None,
    };
    let json = serde_json::to_string(&chunk)?;

    let password = std::env::var("shelly-password")?;

    // let response = send_with_shelly_digest_auth(&url, json, uri, "admin" )?;
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(&url)
        .header("Content-Length", HeaderValue::from(json.len()))
        .body(json)
        .send_with_digest_auth("admin", &password)?;

    //debug!("{:?}", response.text());

    if response.status().is_client_error() {
        return Err(Box::new(ClientRequestError { code: response.status().as_u16() }));
    }
    else if response.status().is_server_error() {
        return Err(Box::new(InternalServerError { code: response.status().as_u16() }));
    }

    Ok(())
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
