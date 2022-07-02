mod launch;
mod tasks;
mod jlink;
mod defaults;
mod paths;

use launch::*;
use tasks::*;
use jlink::*;
use defaults::*;
use paths::*;

fn main() {
    // let projectpaths = ProjectPaths::new();
    // let mut launchconfig = LaunchConfiguration::new();
    // let mut taskconfig = TaskConfiguration::new(true);

    // launchconfig.device = "STM32F103CB".into();
    // taskconfig.label = "make programm".into();

    // let launchfile = LaunchFile::new(vec!(launchconfig));
    // launchfile.create(&projectpaths).unwrap();

    // let taskfile = TasksFile::new(vec!(taskconfig));
    // taskfile.create(&projectpaths).unwrap();

    // let flashfile = FlashFile::new("bin/exe.hex".into());
    // flashfile.create(&projectpaths).unwrap();



    ProjectDefaults::create();

    let project_paths = ProjectPaths::new();
    let mut project_defaults = ProjectDefaults::new();
    project_defaults.load_defaults(&project_paths);

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
    launch_file.create(&project_paths).unwrap();

    let mut task_config = TaskConfiguration::new(project_defaults.task_defaults.is_default);
    task_config.args = project_defaults.task_defaults.args;
    task_config.command = project_defaults.task_defaults.command;
    task_config.label = project_defaults.task_defaults.name;

    let mut task_file = TasksFile::new(vec!(task_config));
    task_file.version = project_defaults.task_defaults.version;
    task_file.create(&project_paths).unwrap();

    for default_jlinkfile in project_defaults.jlink_defaults {
        let mut jlinkfile = JlinkFile::new();
        jlinkfile.filename = default_jlinkfile.filename;
        for command in default_jlinkfile.commands {
            jlinkfile.commands.push( JlinkCommand::from_string(&command) );
        }
        jlinkfile.create(&project_paths).unwrap();
    }
    
    println!("set up project successful, have fun");
    //ProjectDefaults::create();
}
