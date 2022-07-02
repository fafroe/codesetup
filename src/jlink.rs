use std::vec::Vec;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::PathBuf;

use crate::ProjectPaths;

#[allow(unused)]
pub struct JlinkFile {
    pub filename: String,
    pub commands: Vec<JlinkCommand>,
}

#[allow(unused)]
pub enum JlinkCommand {
    Reset,
    Go,
    Exit,
    Halt,
    WaitHalt,
    Flash(PathBuf),
    Sleep(u32),
    Erase,
    Error,
}

#[allow(unused)]
impl JlinkFile {
    pub fn new() -> JlinkFile {
        JlinkFile {
            filename: String::new(),
            commands: vec!(),
        }
    }

    pub fn create(&self, output_dir: &ProjectPaths) -> io::Result<File> {
        // Remove extension if needed and remove correct one
        let filename = PathBuf::from(&self.filename);
        let mut filename = match &filename.file_name() {
            Some(name) => PathBuf::from(name),
            None => PathBuf::from(&self.filename)
        };
        filename.with_extension(".Jlink");

        let mut filepath = output_dir.dot_vscode_dir.clone();
        filepath.push(filename);

        let mut file = match fs::File::create(&filepath) {
            Ok(file) => file,
            Err(error) => panic!("could not create {0:?}: {1:?}", filepath, error.kind()),
        };

        // write commands
        for cmd in &self.commands {
            file.write_all(cmd.to_string(true).as_bytes())?;
        }

        Ok(file)
    }
}
impl JlinkCommand {
    pub fn to_string(&self, addnewline: bool) -> String {
        use JlinkCommand::*;
        let temp = match self {
            Reset       => "Reset".into(),
            Go          => "Go".into(),
            Exit        => "Exit".into(),
            Halt        => "Halt".into(),
            WaitHalt    => "WaitHalt".into(),
            Flash(path) => format!("Flash {}", path.to_str().unwrap()),
            Sleep(time) => format!("Sleep {}", time),
            Erase       => "Erase".into(),
            Error       => "_ERROR_".into()
        };

        if addnewline {
            return format!("{}\n", temp);
        }
        temp.into()
    }

    pub fn from_string(command: &String) -> JlinkCommand {
        use JlinkCommand::*;

        let command = command.to_lowercase();

        let str_command: &str = &command;
        match str_command {
            "reset"     => Reset,
            "erase"     => Erase,
            "go"        => Go,
            "halt"      => Halt,
            "exit"      => Exit,
            "waithalt"  => WaitHalt,
            _ => {
                if command.contains("flash") {
                    let mut split_iter = command.split_whitespace();
                    split_iter.next();
        
                    match split_iter.next() {
                        Some(path) => return Flash(PathBuf::from(path)),
                        None => (),
                    }
                    println!("Warning: No flash path defined");
                    return Sleep(0)
                }
        
                if command.contains("sleep") {
                    let mut split_iter = command.split_whitespace();
                    split_iter.next();
        
                    match split_iter.next() {
                        Some(delay) => {
                            match delay.parse::<u32>() {
                                Ok(n) => return Sleep(n),
                                Err(_) => (),
                            }
                        }
                        None => (),
                    }
                    println!("Warning: No or wrong sleep time defined");
                    return Sleep(0)
                }

                return Error;
            },
        }
    }
}