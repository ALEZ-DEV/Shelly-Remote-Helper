use log::{error, info};
use crate::service::shelly_rest_api::Shelly;
use crate::service::vscode_tasks::SetupVsCode;

pub fn setup(vscode: bool) {
    if vscode {
        let result = SetupVsCode::new();

        if result.is_err() {
            error!("Failed to create the config on initialisation");
            error!("Due to -> {}", result.unwrap_err());
            return;
        }

        let setup_vs_code= result.as_ref().unwrap();

        let write_result = setup_vs_code.write();

        if write_result.is_err() {
            error!("Failed to write the file");
            error!("Due to -> {}", result.unwrap_err());
            return;
        } else {
            info!("Config file created, you can now close this console");
        }
    } else {
        error!("You have to choose a config profile to create the config !")
    }
}

pub fn start(script_name: &str) {

    std::env::set_var("shelly-autorun", true.to_string());

    let result = Shelly::new();
    if result.is_err() {
        error!("oh my... how it can be wrong ? go check main.rs line 96 to 99");
        return;
    }

    let shelly = result.unwrap();

    let list_result = shelly.script_list();
    if list_result.is_err() {
        error!("Failed to get the script list from the Shelly");
        return;
    }

    let script_list = list_result.unwrap();
    let script = script_list
        .iter()
        .filter_map(|script| {
            if script.name == script_name {
                Some(script)
            } else {
                None
            }
        }).next();

    if script.is_none() {
        error!("can't start the script, because is not existent");
        return;
    }

    let start_result = shelly.script_start(script.unwrap());

    if start_result.is_ok() {
        info!("script started !");
    } else {
        error!("Unable to start script");
        error!("Due to -> {}", start_result.unwrap_err())
    }
}