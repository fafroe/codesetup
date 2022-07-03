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
            match codesetup::init(&project_paths) {
                Err(_) => (),
                _ => (),
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
