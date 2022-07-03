use std::io;

use crate::defaults::*;
use crate::paths::*;
use crate::launch::*;
use crate::tasks::*;
use crate::jlink::*;

pub fn init(project_paths: &ProjectPaths) -> io::Result<()>{
    let mut project_defaults = ProjectDefaults::new();
    match project_defaults.load_defaults(&project_paths) {
        Ok(_) => println!("Loading settings from defaults.json"),
        Err(e) => match e.kind() {
            io::ErrorKind::NotFound => {
                println!("defaults.josn not found. Using new settings.");
                project_defaults = ProjectDefaults::new_defaults();
            }
            _ => return Err(e),
        }
    }

    match std::fs::create_dir(project_paths.dot_vscode_dir.clone()) {
        Err(e) => match e.kind() {
            std::io::ErrorKind::AlreadyExists => (),
            _ => return Err(e),
        },
        Ok(_) => (),
    }

    let mut launch_config = LaunchConfiguration::new();
    launch_config.executable = project_defaults.binary_path.clone();
    launch_config.device = project_defaults.controller;
    launch_config.name = project_defaults.launch_defaults.launch_name;
    launch_config.interface = project_defaults.launch_defaults.interface;
    launch_config.servertype = project_defaults.launch_defaults.server_type;
    launch_config.serverpath = project_defaults.launch_defaults.server_path;
    launch_config.pre_launch_commands = project_defaults.launch_defaults.prelaunch_commands;

    let mut launch_file = LaunchFile::new(vec!(launch_config));
    launch_file.version = project_defaults.launch_defaults.version;
    launch_file.create(&project_paths)?;

    let mut task_config = TaskConfiguration::new();
    task_config.group.is_default = project_defaults.task_defaults.is_default;
    task_config.args = project_defaults.task_defaults.args;
    task_config.command = project_defaults.task_defaults.command;
    task_config.tasktype = project_defaults.task_defaults.tasktype;
    task_config.label = project_defaults.task_defaults.name;

    let mut task_file = TasksFile::new();
    task_file.append_task(task_config);
    task_file.version = project_defaults.task_defaults.version;
    task_file.create(&project_paths)?;

    for default_jlinkfile in project_defaults.jlink_defaults {
        let mut jlinkfile = JlinkFile::new();
        jlinkfile.filename = default_jlinkfile.filename;
        for command in default_jlinkfile.commands {
            jlinkfile.commands.push( JlinkCommand::from_string(&command) );
        }
        jlinkfile.create(&project_paths)?;
    }

    println!("Codesetup init done.");
    Ok(())
}

pub fn install() {
    ProjectDefaults::create();
}

pub fn help(project_paths: &ProjectPaths) {
    println!("Codesetup");
    println!("\tenables easy code setup for ARM Cortex projects.\n");

    println!("USAGE:");
    println!("\tcodesetup [COMMAND]");
    println!("");

    println!("COMMANDS:");
    println!("\t init\tsettting up actual project for VSCode");
    println!("\t install\tcreats default files in homefolder/.codesetup");
    println!("");

    println!("SETTINGS LOCATION:");
    println!("\t\"{}\"", &project_paths.codesetup_settings_dir.to_string_lossy());
    println!("");
}