use std::collections::HashMap;
use std::error::Error;
use std::thread;
use std::fs;
use std::time::Duration;
use filetime::FileTime;
use log::{debug, error};
use crate::service::shelly_rest_api::save_script_to_shelly;

pub(crate) struct FileChecker {
    last_modified: HashMap<String, FileTime>,
}

impl FileChecker {
    pub fn new() -> Self {
        FileChecker {
            last_modified: HashMap::new(),
        }
    }

    pub fn start(&mut self, path_to_check: &str) {

        loop {
            let result = self.check_directory(path_to_check);

            match result {
                Err(error) => {
                    error!("Somthing goes wrong when processing files in the directory -> {}", error);
                }
                _ => {}
            }

            thread::sleep(Duration::from_millis(500));
        }
    }

    fn check_directory(&mut self, directory_to_check: &str) -> Result<(), Box<dyn Error>>{

        let files_path = fs::read_dir(directory_to_check)?;

        for file in files_path {
            let file_info = file?;
            let file_path = file_info.path();
            let path = file_path.to_str().ok_or("")?;

            let metadata = fs::metadata(path)?;

            if metadata.is_file() {
                let modif_time = FileTime::from_last_modification_time(&metadata);
                let last_time_result= self.last_modified.get(path);

                match last_time_result {
                    None => { self.last_modified.insert(path.to_string(), modif_time); }
                    Some(last_modif_time) => {
                        if &modif_time > last_modif_time {
                            debug!("{} has been modified !", &file_info.path().display());
                            self.last_modified.insert(path.to_string(), modif_time);
                            save_script_to_shelly(path)?;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

