use std::io;

use crate::defaults::*;
use crate::paths::*;
use crate::launch::*;
use crate::tasks::*;
use crate::jlink::*;

pub fn init(project_paths: &ProjectPaths, controller: String) -> io::Result<()>{
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
            io::ErrorKind::AlreadyExists => (),
            _ => return Err(e),
        },
        Ok(_) => (),
    }

    let mut launch_file = LaunchFile::new();
    let mut task_file = TasksFile::new();
    let mut does_setting_exist = false;

    for settings in &project_defaults.controller_defaults {
        if settings.controller == controller {
            does_setting_exist = true;

            let mut launch_config = LaunchConfiguration::new();
            launch_config.executable      = settings.binary_path.clone();
            launch_config.device          = settings.controller.clone();
            launch_config.name            = settings.launch_defaults.launch_name.clone();
            launch_config.interface       = settings.launch_defaults.interface.clone();
            launch_config.pre_launch_task = settings.launch_defaults.prelaunch_task.clone();
            launch_config.serverpath      = project_defaults.jlink_path.clone();
            launch_file.append_launch_config(launch_config);
            launch_file.create(&project_paths)?;

            for task in &settings.task_defaults {
                let mut task_config = TaskConfiguration::new();
                task_config.group.is_default = task.is_default;
                task_config.args             = task.args.clone();
                task_config.command          = task.command.clone();
                task_config.label            = task.task_name.clone();
                task_file.append_task(task_config);
            }
            task_file.create(&project_paths)?;

            for jlink_content in &settings.jlink_defaults {
                let mut jlinkfile = JlinkFile::new();
                jlinkfile.filename = jlink_content.filename.clone();
                for command in &jlink_content.commands {
                    jlinkfile.commands.push( JlinkCommand::from_string(&command) );
                }
                jlinkfile.create(&project_paths)?;
            }
        }
    }

    if does_setting_exist == false {
        return Err(io::Error::from(io::ErrorKind::NotFound));
    }

    Ok(())
}

pub fn install() {
    ProjectDefaults::create();
}

pub fn help(project_paths: &ProjectPaths) {
    println!("Codesetup");
    println!("\tenables easy code setup for ARM Cortex projects.\n");

    println!("USAGE:");
    println!("\tcodesetup [COMMAND] [PARAMS]");
    println!("");

    println!("COMMANDS:");
    println!("\t init\tsettting up actual project for VSCode");
    println!("\t install\tcreats default files in homefolder/.codesetup");
    println!("");

    println!("SETTINGS LOCATION:");
    println!("\t\"{}\"", &project_paths.codesetup_settings_dir.to_string_lossy());
    println!("");
}