use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::io::ErrorKind;
use std::vec;

use crate::paths::*;


#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchFile {
    pub version: String,
    pub configurations: Vec<LaunchConfiguration>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchConfiguration {
    pub name: String,
    pub request: String,
    #[serde(rename = "type")]
    pub configtype: String,
    pub cwd: String,
    pub executable: String,
    pub serverpath: String,
    pub servertype: String,
    pub interface: String,
    pub device: String,
    #[serde(rename = "runToEntryPoint")]
    pub run_to_entry_point: String,
    #[serde(rename = "showDevDebugOutput")]
    pub show_dev_debug_output: String,
    #[serde(rename = "preLaunchCommands")]
    pub pre_launch_task: String,
}

#[allow(dead_code)]
impl LaunchFile {
    pub fn new() -> LaunchFile{
        LaunchFile {
            version: "0.2.0".into(),
            configurations: vec::Vec::new(),
        }
    }

    pub fn append_launch_config(&mut self, launch_config: LaunchConfiguration) {
        self.configurations.push(launch_config);
    }

    pub fn create(&self, project_paths: &ProjectPaths) -> io::Result<File> {
        match fs::create_dir(project_paths.dot_vscode_dir.clone()) {
            Err(e) => match e.kind() {
                ErrorKind::AlreadyExists => (),
                ErrorKind::PermissionDenied => (),
                _ => {
                    println!("creating folder failed");
                    return Err(e);
                },
            },
            Ok(_) => (),
        }

        let mut file = match fs::File::create(project_paths.launch_file.clone()) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let json = serde_json::to_string_pretty(self)?;
        file.write_all( json.as_bytes() )?;
        
        Ok(file)
    }
}

#[allow(dead_code)]
impl LaunchConfiguration {
    pub fn new() -> LaunchConfiguration{
        LaunchConfiguration {
            name: String::new(),
            request: "launch".into(),
            configtype: "cortex-debug".into(),
            cwd: "${workspacefolder}".into(),
            executable: ".build/out.bin".into(),
            serverpath: String::new(),
            servertype: "jlink".into(),
            interface: String::new(),
            device: String::new(),
            run_to_entry_point: "main".into(),
            show_dev_debug_output: "none".into(),
            pre_launch_task: String::new(),
        }
    }
}