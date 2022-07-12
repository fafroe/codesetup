use std::vec;
use serde::{Serialize, Deserialize};

use std::path::Path;
use std::fs;
use std::io::Write;

use crate::paths::*;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectDefaults {
    pub jlink_path: String,
    pub controller_defaults: vec::Vec<ControllerDefaults>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerDefaults {
    pub controller: String,
    pub binary_path: String,
    pub launch_defaults: LaunchDefaults,
    pub task_defaults: vec::Vec<TasksDefaults>,
    pub jlink_defaults: vec::Vec<JlinkDefaults>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchDefaults {
    pub launch_name: String,
    pub interface: String,
    pub prelaunch_task: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TasksDefaults {
    pub task_name: String,
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
            Err(error) => panic!("Could not create string for defaults jason: {:?}", error),
        };

        match defaults_file.write_all( json_string.as_bytes() ) {
            Ok(_) => (),
            Err(error) => panic!("could not write to defaults.json: {:?}", error.kind()), 
        }

        println!("Created defauls.json successfuly");
    }

    pub fn load_defaults(&mut self, project_paths: &ProjectPaths) -> std::io::Result<()> {
        let defaults = fs::read_to_string(project_paths.defaults_file.clone())?;

        let  defaults: ProjectDefaults = serde_json::from_str(&defaults)?;

        self.jlink_path             = defaults.jlink_path;
        self.controller_defaults    = defaults.controller_defaults;
        Ok(())
    }

    pub fn new() -> ProjectDefaults {
        let defaults = ProjectDefaults {
            jlink_path: String::new(),
            controller_defaults: vec::Vec::new(),
        };

        defaults
    }

    pub fn new_defaults() -> ProjectDefaults {
        let os_depnedend_jlink_path = match std::env::consts::FAMILY {
            "windows" => String::from("C:\\Program Files (x86)\\SEGGER\\JLink\\JLinkGDBServerCL.exe"),
            _ => String::from("/opt/SEGGER/JLink/JLinkGDBServerCLExe")
        };

        let defaults = ProjectDefaults {
            jlink_path: os_depnedend_jlink_path,
            controller_defaults: vec!(
                ControllerDefaults {
                    controller: "generic".into(),
                    binary_path: "debug/out.elf".into(),
                    jlink_defaults: vec!(
                        JlinkDefaults {
                            filename: "erase".into(),
                            commands: vec!(
                                "Reset".into(),
                                "Sleep 100".into(),
                                "Erase".into(),
                                "Exit".into()
                            ),
                        },
                        JlinkDefaults {
                            filename: "flash".into(),
                            commands: vec!(
                                "Reset".into(),
                                "Sleep 100".into(),
                                "Erase".into(),
                                "Sleep 100".into(),
                                "LoadFile debug/out.elf".into(),
                                "Sleep 100".into(),
                                "Reset".into(),
                                "Sleep 100".into(),
                                "Go".into(),
                                "Exit".into()
                            ),
                        }
                    ),
                    launch_defaults: LaunchDefaults {
                        interface: "swd".into(),
                        launch_name: "Debug with Jlink/GDB".into(),
                        prelaunch_task: "make".into(),
                    },
                    task_defaults: vec!(
                        TasksDefaults {
                            task_name: "make binary".into(),
                            command: "make".into(),
                            args: vec::Vec::new(),
                            is_default: true,
                        },
                    )
                },
            )
        };
        defaults
    }

}
