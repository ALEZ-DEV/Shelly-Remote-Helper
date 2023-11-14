use std::error::Error;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::fs::read_to_string;
use std::path::Path;
use log::{debug, error, info};
use reqwest::header::{HeaderValue};
use serde::{Deserialize, Serialize};
use diqwest::blocking::WithDigestAuth;

///JavaScript function that can be called in the js code to stop the current file
///
/// This code is automatically append to an end of a file
static JS_STOP_FUNCTION: &str = "\n
function stopCurrentScript() {
    let SCRIPT_ID = Shelly.getCurrentScriptId();
    let msg = \"script \" + SCRIPT_ID + \" has been stopped\";
    print(msg);
    Shelly.call(
        \"Script.Stop\",
        { \"id\" : SCRIPT_ID},
        function (result, error_code, error_message, usedata) {}
    );
};
";

///Upload and save file to the Shelly by the file path
///
/// * `save_script_to_shelly` - the path of the file to upload to the shelly
pub fn save_script_to_shelly(file_path: &str) -> Result<(), Box<dyn Error>>{
    let file_content = read_to_string(file_path)?;
    let file_name = Path::new(file_path).file_name()
        .ok_or(OsStr::new("/"))
        .unwrap()
        .to_str()
        .ok_or("What's the name of this file guys ?")?
        .replace(".js", "");

    if !file_path.contains(".js") {
        return Ok(());
    }

    debug!("file name : {file_name}");

    let shelly = Shelly::new()?;
    let available_scripts = shelly.script_list()?;
    debug!("{:?}", available_scripts);

    let script = available_scripts
        .iter()
        .filter_map(|script| {
            if script.name == file_name {
                Some(script)
            } else {
                None
            }
        }).next();
    debug!("{:?}", script);

    if script.is_none() {
        let new_script = shelly.script_create(&file_name)?;
        shelly.script_put_code(&new_script, file_content, false)?;
        shelly.script_start(&new_script)?;
    } else {
        let script_u = script.unwrap();
        shelly.script_stop(script_u)?;
        shelly.script_put_code(script_u, file_content, false)?;
        shelly.script_start(script_u)?;
    }

    info!("The file has been correctly uploaded !");

    Ok(())
}

///Representation of a script on the Shelly
#[derive(Serialize, Deserialize, Debug)]
pub struct Script {
    ///The current id of the script on the Shelly
    pub id: i32,
    ///The current name of the script on the Shelly
    pub name: String,
    ///The status if the script is currently enable or not on the Shelly, can be [None]
    pub enable: Option<bool>,
    ///The status if the script is currently running or not on the Shelly can be [None]
    pub running: Option<bool>,
}

///A chunk is a part of the code that will be send to the Shelly
#[derive(Serialize, Deserialize)]
struct Chunk {
    ///The script where the [Chunk] have to be send
    id: i32,
    ///The code that will be send to the Shelly
    code: String,
    ///If [true] will append the code to the existing one, if [false] will override it
    append: bool,
}

///Define the Shelly structure and data, can create an instance of it with [Shelly::new()]
#[derive(Debug)]
pub struct Shelly {
    ///The Http client who will communicate with the Shelly
    client: reqwest::blocking::Client,
    ///The IP of the Shelly
    host: String,
    ///The username to be able to connect to the Shelly
    username: String,
    ///The password to be able to connect to the Shelly
    password: String,
}

impl Shelly {
    ///Create a new instance of Shelly
    ///
    /// Return an error if something goes wrong
    pub fn new() -> Result<Self, Box<dyn Error>>{
        Ok(Shelly {
                client: reqwest::blocking::Client::new(),
                host: std::env::var("shelly-host")?,
                username: std::env::var("shelly-username")?,
                password: std::env::var("shelly-password")?,
            })
    }

    ///Generate the url to access the Shelly API
    fn get_url(&self, uri: &str) -> String {
        format!("http://{}{uri}", self.host)
    }

    ///Create and upload the script to the shelly
    ///
    /// * `script_name` - The name of the script that will be upload
    ///
    ///Equivalent to http://{shelly_ip}/rpc/Script.Create
    ///
    ///Return an error if something goes wrong
    fn script_create(&self, script_name: &str) -> Result<Script, Box<dyn Error>> {
        let uri = "/rpc/Script.Create";
        let url = self.get_url(uri);

        let json = format!("{{ \"name\": \"{}\" }}", script_name);

        let response = self.client
            .post(&url)
            .header("Content-Length", HeaderValue::from(json.len()))
            .body(json)
            .send_with_digest_auth(&self.username, &self.password)?;

        if response.status().is_client_error() {
            let error_code = response.status().as_u16();
            let body = response.text()?;
            error!("{}", &body);

            return Err(Box::new(ClientRequestError { code: error_code }));
        }
        else if response.status().is_server_error() {
            let id = response.status().as_u16().clone();
            let msg = response.text();

            let mut checked_msg = None;
            if msg.is_ok() {
                checked_msg = Some(msg.unwrap())
            }

            return Err(Box::new(InternalServerError {
                code: id,
                msg: checked_msg,
            }));
        }

        let body = response.text()?;
        debug!("{body}");
        let id: serde_json::Value = serde_json::from_str(&body)?;

        Ok(Script {
            id: id["id"].as_i64().ok_or(-1).unwrap() as i32,
            name: script_name.to_string(),
            enable: None,
            running: Some(false),
        })
    }

    ///Will override all the code in the indicate Script
    ///
    /// * `script` - The script to send the code
    /// * `data` - The code that will be upload to the Shelly
    /// * `append` - If [true] append the code at the end of the file, If [false] will override all existing code
    ///
    ///Equivalent to http://{shelly_ip}/rpc/Script.PutCode
    ///
    /// Return an Error if something goes wrong
    fn script_put_code(&self, script: &Script, data: String, append: bool) -> Result<(), Box<dyn Error>>{
        let uri = "/rpc/Script.PutCode";
        let url = self.get_url(uri);
        debug!("{}", url);

        let chunk = Chunk {
            id: script.id,
            code: format!("{data}{JS_STOP_FUNCTION}"),
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
            let id = response.status().as_u16().clone();
            let msg = response.text();

            let mut checked_msg = None;
            if msg.is_ok() {
                checked_msg = Some(msg.unwrap())
            }

            return Err(Box::new(InternalServerError {
                code: id,
                msg: checked_msg,
            }));
        }

        Ok(())
    }

    ///Will return a list of the currents `Vec<Script>` with their current status
    ///
    ///Return an Error if something goes wrong
    pub fn script_list(&self) -> Result<Vec<Script>, Box<dyn Error>> {
        let uri = "/rpc/Script.List";
        let url = self.get_url(uri);

        let response = self.client
            .get(&url)
            .send_with_digest_auth(&self.username, &self.password)?;

        if response.status().is_client_error() {
            return Err(Box::new(ClientRequestError { code: response.status().as_u16() }));
        }
        else if response.status().is_server_error() {
            let id = response.status().as_u16().clone();
            let msg = response.text();

            let mut checked_msg = None;
            if msg.is_ok() {
                checked_msg = Some(msg.unwrap())
            }

            return Err(Box::new(InternalServerError {
                code: id,
                msg: checked_msg,
            }));
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

    ///Will start a script with the current name on the Shelly
    ///
    ///  * `script` - The [Script] to start
    ///
    ///Return an Error if something goes wrong
    pub fn script_start(&self, script: &Script) -> Result<(), Box<dyn Error>> {
        let autorun = std::env::var("shelly-autorun")?.parse::<bool>()?;
        if script.running.unwrap() && !autorun {
            return Ok(());
        }

        let uri = "/rpc/Script.Start";
        let url = self.get_url(uri);

        let json = format!("{{ \"id\": \"{}\" }}", script.id);

        let response = self.client
            .post(&url)
            .header("Content-Length", HeaderValue::from(json.len()))
            .body(json)
            .send_with_digest_auth(&self.username, &self.password)?;

        if response.status().is_client_error() {
            let error_code = response.status().as_u16();
            let body = response.text()?;
            error!("{}", &body);

            return Err(Box::new(ClientRequestError { code: error_code }));
        }
        else if response.status().is_server_error() {
            let id = response.status().as_u16().clone();
            let msg = response.text();

            let mut checked_msg = None;
            if msg.is_ok() {
                checked_msg = Some(msg.unwrap())
            }

            return Err(Box::new(InternalServerError {
                code: id,
                msg: checked_msg,
            }));
        }

        Ok(())
    }

    ///Will stop a script with the current name on  the shelly
    ///
    /// * `script` - the [Script] that will be stop
    ///
    ///Return an Error if something goes wrong
    pub fn script_stop(&self, script: &Script) -> Result<(), Box<dyn Error>> {
        if !script.running.unwrap() {
            return Ok(());
        }

        let uri = "/rpc/Script.Stop";
        let url = self.get_url(uri);

        let json = format!("{{ \"id\": \"{}\" }}", script.id);

        let response = self.client
            .post(&url)
            .header("Content-Length", HeaderValue::from(json.len()))
            .body(json)
            .send_with_digest_auth(&self.username, &self.password)?;

        if response.status().is_client_error() {
            let error_code = response.status().as_u16();
            let body = response.text()?;
            error!("{}", &body);

            return Err(Box::new(ClientRequestError { code: error_code }));
        }
        else if response.status().is_server_error() {
            let id = response.status().as_u16().clone();
            let msg = response.text();

            let mut checked_msg = None;
            if msg.is_ok() {
                checked_msg = Some(msg.unwrap())
            }

            return Err(Box::new(InternalServerError {
                code: id,
                msg: checked_msg,
            }));
        }

        Ok(())
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
    msg: Option<String>,
}
impl Error for InternalServerError {}
impl Display for InternalServerError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut msg = format!("There is something wrong going on with the shelly -> response code {}", self.code);
        if self.msg.is_some() {
            msg = format!("{}\nError message -> {}", msg, self.msg.clone().unwrap());
        }
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


