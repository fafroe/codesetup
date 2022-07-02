use std::path::PathBuf;

#[allow(unused)]
pub struct ProjectPaths {
    pub dot_vscode_dir: PathBuf,
    pub launch_file: PathBuf,
    pub flash_file: PathBuf,
    pub tasks_file: PathBuf,

    pub codesetup_settings_dir: PathBuf,
    pub defaults_file: PathBuf,
}

#[allow(unused)]
impl ProjectPaths {
    pub fn new() -> ProjectPaths {
        let mut _codesetup_settings_dir = match dirs::home_dir() {
            Some(path) => path,
            None => panic!("could not find home directory")
        };
        _codesetup_settings_dir.push(".codesetup");
        
        let mut _defaults_file = _codesetup_settings_dir.clone();
        _defaults_file.push("defaults.json");

        let _dot_vscode_dir = PathBuf::from(".vscode_test");

        let mut _launch_file = _dot_vscode_dir.clone();
        _launch_file.push("launch.json");

        let mut _flash_file = _dot_vscode_dir.clone();
        _flash_file.push("flash.jlink");

        let mut _tasks_file = _dot_vscode_dir.clone();
        _tasks_file.push("tasks.json");

        ProjectPaths {
            codesetup_settings_dir: _codesetup_settings_dir,
            defaults_file: _defaults_file,
            dot_vscode_dir: _dot_vscode_dir,
            launch_file: _launch_file,
            flash_file: _flash_file,
            tasks_file: _tasks_file,
        }
    }

}

