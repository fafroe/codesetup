use std::vec;
use serde::{Serialize, Deserialize};

use std::path::Path;
use std::fs;
use std::io::Write;

use crate::paths::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDefaults {
    pub controller: String,
    pub binary_path: String,
    pub launch_defaults: LaunchDefaults,
    pub task_defaults: TasksDefaults,
    pub jlink_defaults: vec::Vec<JlinkDefaults>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchDefaults {
    pub version: String,
    pub launch_name: String,
    pub server_type: String,
    pub server_path: String,
    pub interface: String,
    pub prelaunch_commands: vec::Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksDefaults {
    pub version: String,
    pub name: String,
    #[serde(rename = "type")]
    pub tasktype: String,
    pub command: String,
    pub args: vec::Vec<String>,
    pub is_default: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JlinkDefaults {
    pub filename: String,
    pub commands: vec::Vec<String>,
}

#[allow(unused)]
impl ProjectDefaults {
    pub fn create() {
        let bin_home_folder = match dirs::home_dir() {
            Some(path) => path.as_path().join(Path::new(".codesetup")),
            None => panic!("could not find home directory")
        };
        let defaults_file_path = bin_home_folder.join("defaults.json");
        match fs::create_dir_all(bin_home_folder) {
            Ok(file) => file,
            Err(error) => panic!("could not create defaults folder: {:?}", error.kind()),
        };

        let defaults = ProjectDefaults::new_defaults();

        let mut defaults_file = match fs::File::create(defaults_file_path) {
            Ok(file) => file,
            Err(error) => panic!("Could not create defaults file: {:?}", error.kind()),
        };

        let json_string = match serde_json::to_string_pretty(&defaults) {
            Ok(data) => data,
            Err(error) => panic!("Could not creat write string for defaults jason: {:?}", error),
        };

        match defaults_file.write_all( json_string.as_bytes() ) {
            Ok(_) => (),
            Err(error) => panic!("could not write defaults.json: {:?}", error.kind()), 
        }
    }

    pub fn load_defaults(&mut self, project_paths: &ProjectPaths) {
        let defaults = match fs::read_to_string(project_paths.defaults_file.clone()) {
            Ok(content) => content,
            Err(err) => panic!("could not read from defaults.json: {:?}", err.kind()),
        };

        let  defaults: ProjectDefaults = serde_json::from_str(&defaults).unwrap();

        self.binary_path     = defaults.binary_path;
        self.controller      = defaults.controller;
        self.jlink_defaults  = defaults.jlink_defaults;
        self.launch_defaults = defaults.launch_defaults;
        self.task_defaults   = defaults.task_defaults;
    }

    pub fn new() -> ProjectDefaults {
        let defaults = ProjectDefaults {
            binary_path: String::new(),
            controller: String::new(),
            jlink_defaults: vec::Vec::new(),
            launch_defaults: LaunchDefaults {
                interface: String::new(),
                server_path: String::new(),
                server_type: String::new(),
                launch_name: String::new(),
                prelaunch_commands: vec::Vec::new(),
                version: String::new(),
            },
            task_defaults: TasksDefaults {
                version: String::new(),
                name: String::new(),
                tasktype: String::new(),
                command: String::new(),
                args: vec::Vec::new(),
                is_default: true,
            },
        };

        defaults
    }

    pub fn new_defaults() -> ProjectDefaults {
        let defaults = ProjectDefaults {
            binary_path: "debug/out.bin".into(),
            controller: "generic".into(),
            jlink_defaults: vec!(
                JlinkDefaults {
                    filename: "erase".into(),
                    commands: vec!(
                        "Erase".into(),
                        "Sleep 100".into(),
                        "Exit".into()
                    ),
                },
                JlinkDefaults {
                    filename: "flash".into(),
                    commands: vec!(
                        "Flash somthing.elf".into(),
                        "Sleep 100".into(),
                        "Exit".into()
                    ),
                }
            ),
            launch_defaults: LaunchDefaults {
                interface: "swd".into(),
                server_path: "C:/Program Files (x86)/SEGGER/JLink/JLinkGDBServerCL.exe".into(),
                server_type: "jlink".into(),
                launch_name: "Debug with Jlink/GDB".into(),
                prelaunch_commands: vec!("make".into()),
                version: "0.2.0".into(),
            },
            task_defaults: TasksDefaults {
                version: "2.0.0".into(),
                name: "make binary".into(),
                tasktype: "shell".into(),
                command: "build".into(),
                args: vec::Vec::new(),
                is_default: true,
            },
        };

        defaults
    }

}
