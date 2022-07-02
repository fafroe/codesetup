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
pub struct TasksFile {
    pub version: String,
    pub tasks: Vec<TaskConfiguration>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskConfiguration {
    pub label: String,
    #[serde(rename = "type")]
    pub tasktype: String,
    pub command: String,
    pub args: Vec<String>,
    group: TaskGroup,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct TaskGroup {
    pub kind: String,
    #[serde(rename = "isDefault")]
    pub is_default: bool,
}

#[allow(dead_code)]
impl TasksFile {
    //set default Values
    pub fn new(tasks: vec::Vec<TaskConfiguration>) -> TasksFile{
        TasksFile {
            version: "2.0.0".into(),
            tasks: tasks,
        }
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

        let mut file = match fs::File::create(project_paths.tasks_file.clone()) {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let json = serde_json::to_string_pretty(self)?;
        file.write_all( json.as_bytes() )?;
        
        Ok(file)
    }
}

#[allow(dead_code)]
impl TaskConfiguration {
    // set default Values
    pub fn new(default: bool) -> TaskConfiguration{
        let tempgroup = TaskGroup {
            kind: "build".into(),
            is_default: default,
        };

        TaskConfiguration {
            label: "compile".into(),
            tasktype: "shell".into(),
            command: "make".into(),
            args: Vec::new(),
            group: tempgroup,
        }
    }
}