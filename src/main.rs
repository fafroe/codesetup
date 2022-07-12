mod launch;
mod tasks;
mod jlink;
mod defaults;
mod paths;
mod codesetup;

use std::env;

use paths::*;

fn main() {
    println!("Codesetup launched");
    let args: Vec<String> = env::args().collect();
    let project_paths = ProjectPaths::new();

    if args.len() > 1 {
        if args[1] == String::from("init") {
            //init specified setting
            if args.len() > 2 {
                match codesetup::init(&project_paths, args[2].clone()) {
                    Err(err) => match err.kind() {
                        std::io::ErrorKind::NotFound => println!("Error: Could not find controller settings: {}", args[2]),
                        std::io::ErrorKind::InvalidData => println!("Error: Invalid data while reading defaults.json: {}", err),
                        _ => panic!("Error: Failed to init project: {}", err),
                    }
                    _ => println!("Codesetp using {} settings for init.", args[2]),
                }
            } else {
                //init defaults
                match codesetup::init(&project_paths, "generic".into()) {
                    Err(err) => panic!("Error: Failed to init project: {}", err),
                    _ => println!("Codesetp using generic settings for init."),
                }
                println!("Codesetup init done.");
            }
        }
        if args[1] == String::from("install") {
            codesetup::install();
        }
        if args[1] == String::from("help") {
            codesetup::help(&project_paths);
        }
    }
    else {
        println!("Error: No arguments passed. Displaying help:\n");
        codesetup::help(&project_paths);
    }
}
