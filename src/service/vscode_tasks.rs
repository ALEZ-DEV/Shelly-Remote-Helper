use serde::{Deserialize, Serialize};

struct Generator {
    tasks: Tasks,
}

impl Generator {
    fn new() -> Self {
        Generator {
            tasks: Tasks {
                version: String::from("2.0.0"),
                tasks: vec![
                    Task {
                        label: String::from("Shelly Remote Helper | start Debug"),
                        
                    }
                ]
            }
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Tasks {
    version: String,
    tasks: Vec<Task>,
}

#[derive(Serialize, Deserialize)]
struct Task {
    label: String,
    #[serde(rename = "type")]
    _type: String,
    command: String,

}

#[derive(Serialize, Deserialize)]
struct Presentation {
    reveal: String,
    panel: String,
}

#[derive(Serialize, Deserialize)]
struct runOptions {
    runOn: String,
}