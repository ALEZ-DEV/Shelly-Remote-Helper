use std::error::Error;
use std::fs::File;
use std::io::Write;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct SetupVsCode {
    tasks: Tasks,
}

impl SetupVsCode {
    pub fn new() -> Result<Self, Box<dyn Error>>{
        let host = std::env::var("shelly-host")?;
        let username = std::env::var("shelly-username")?;
        let password = std::env::var("shelly-password")?;

        Ok(
            SetupVsCode {
                tasks: Tasks {
                    version: String::from("2.0.0"),
                    tasks: vec![
                        Task {
                            label: String::from("Shelly Remote Helper | start Debug"),
                            _type: String::from("shell"),
                            command: format!("${{workspaceFolder}}/.vscode/Shelly_Remote_Helper.exe --path ${{workspaceFolder}} --host {host} --password {password} --username {username}"),
                            group: String::from("none"),
                            presentation: Presentation {
                                reveal: String::from("always"),
                                panel: String::from("new"),
                            },
                            run_option: Some(RunOptions {
                                run_on: String::from("folderOpen"),
                            }),
                        },
                        Task {
                            label: String::from("Shelly Remote Helper | start current Script"),
                            _type: String::from("shell"),
                            command: format!("${{workspaceFolder}}/.vscode/Shelly_Remote_Helper.exe start ${{fileBasenameNoExtension}} --host {host} --password {password} --username {username}"),
                            group: String::from("none"),
                            presentation: Presentation {
                                reveal: String::from("always"),
                                panel: String::from("new"),
                            },
                            run_option: None,
                        },
                    ],
                },
            }
        )
    }

    pub fn write(&self) -> Result<(), Box<dyn Error>> {
        let json_tasks = serde_json::to_string_pretty(&self.tasks)?;
        let mut file = File::create("./tasks.json")?;
        file.write(json_tasks.as_bytes())?;

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Tasks {
    version: String,
    tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Task {
    label: String,
    #[serde(rename = "type")]
    _type: String,
    command: String,
    group: String,
    presentation: Presentation,
    #[serde(skip_serializing_if = "Option::is_none")]
    run_option: Option<RunOptions>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Presentation {
    reveal: String,
    panel: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RunOptions {
    run_on: String,
}