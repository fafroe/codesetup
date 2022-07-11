use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
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
    pub group: TaskGroup,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskGroup {
    pub kind: String,
    pub is_default: bool,
}

#[allow(dead_code)]
impl TasksFile {
    pub fn new() -> TasksFile{
        TasksFile {
            version: "2.0.0".into(),
            tasks: vec::Vec::new(),
        }
    }

    pub fn create(&self, project_paths: &ProjectPaths) -> io::Result<File> {
        let mut file = fs::File::create(project_paths.tasks_file.clone())?;
        let json = serde_json::to_string_pretty(&self)?;
        file.write_all( json.as_bytes() )?;
        Ok(file)
    }

    pub fn append_task(&mut self, task: TaskConfiguration) {
        self.tasks.push(task);
    }
}

#[allow(dead_code)]
impl TaskConfiguration {
    pub fn new() -> TaskConfiguration{
        let tempgroup = TaskGroup {
            kind: String::new(),
            is_default: false,
        };

        TaskConfiguration {
            label: String::new(),
            tasktype: "shell".into(),
            command: String::new(),
            args: Vec::new(),
            group: tempgroup,
        }
    }
}