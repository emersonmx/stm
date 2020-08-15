use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::{Path, PathBuf};
use std::{fs::File, io::BufReader};

pub fn app_dir() -> PathBuf {
    dirs::config_dir().unwrap().join("stm")
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Config {
    pub managers: ManagerList,
    pub tools: ToolList,
}

impl Config {
    fn path() -> PathBuf {
        app_dir().join("config.json")
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn default() -> Result<Config, Box<dyn Error>> {
        let file = File::open(Config::path())?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn find_manager(&self, name: &str) -> Option<&Manager> {
        self.managers.0.iter().find(|m| name == m.name)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Manager {
    pub name: String,
    pub install_command: String,
    pub update_command: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ManagerList(Vec<Manager>);

impl ManagerList {
    pub fn names(&self) -> Vec<&String> {
        self.0.iter().map(|m| &m.name).collect()
    }
}

impl Manager {
    pub fn new(name: &str, install_command: &str, update_command: &str) -> Self {
        Self {
            name: String::from(name),
            install_command: String::from(install_command),
            update_command: String::from(update_command),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Tool {
    pub package: String,
    pub binary: Option<String>,
    pub path: Option<String>,
    pub manager: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ToolList(Vec<Tool>);

impl ToolList {
    pub fn filter_by_manager(&self, manager: &str) -> Vec<&Tool> {
        self.0.iter().filter(|t| manager == t.manager).collect()
    }
}

impl Tool {
    fn new(package: &str, binary: Option<&str>, path: Option<&str>, manager: &str) -> Self {
        Self {
            package: String::from(package),
            binary: Some(String::from(binary.unwrap_or(""))),
            path: Some(String::from(path.unwrap_or(""))),
            manager: String::from(manager),
        }
    }

    pub fn new_binary(package: &str, binary: &str, manager: &str) -> Self {
        Self::new(package, Some(binary), None, manager)
    }

    pub fn new_path(package: &str, path: &str, manager: &str) -> Self {
        Self::new(package, None, Some(path), manager)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_temp_json() -> tempfile::NamedTempFile {
        let managers = ManagerList(vec![
            Manager::new("arch", "yay -Sy {{packages}}", "yay -Syu"),
            Manager::new(
                "cargo",
                "cargo install --force {{packages}}",
                "cargo install --force {{packages}}",
            ),
            Manager::new("misc", "misc.sh install", "misc.sh update"),
        ]);
        let tools = ToolList(vec![
            Tool::new_binary("alacritty", "alacritty", "arch"),
            Tool::new_path(
                "ttf-fira-code",
                "/usr/share/fonts/TTF/FiraCode-Regular.ttf",
                "arch",
            ),
            Tool::new_path("cargo-watch", "$CARGO_HOME/bin/cargo-watch", "cargo"),
        ]);
        let config = Config { managers, tools };
        let tf = tempfile::NamedTempFile::new().unwrap();
        let writer = std::io::BufWriter::new(&tf);
        serde_json::to_writer(writer, &config).unwrap();

        tf
    }

    #[test]
    fn it_has_app_dir() {
        let wants = dirs::config_dir().unwrap().join("stm");
        assert_eq!(wants, app_dir());
    }

    #[test]
    fn it_has_config_path() {
        let want = app_dir().join("config.json");
        assert_eq!(want, Config::path());
    }

    #[test]
    fn it_loads_from_json() {
        let tf = create_temp_json();
        let p = tf.path();
        let r = BufReader::new(File::open(p).unwrap());

        let want: Config = serde_json::from_reader(r).unwrap();
        assert_eq!(want, Config::from_file(p).unwrap());
    }

    #[test]
    fn it_has_a_manager_list_with_names() {
        let tf = create_temp_json();
        let p = tf.path();
        let c = Config::from_file(p).unwrap();

        let want = vec!["arch", "cargo", "misc"];
        assert_eq!(want, c.managers.names());
    }

    #[test]
    fn it_finds_a_manager_by_name() {
        let tf = create_temp_json();
        let p = tf.path();
        let c = Config::from_file(p).unwrap();

        let want = c.managers.0.first();
        assert_eq!(want, c.find_manager("arch"));

        let want = c.managers.0.last();
        assert_eq!(want, c.find_manager("misc"));

        let want = None;
        assert_eq!(want, c.find_manager("rust"));
    }

    #[test]
    fn it_finds_tools_by_manager() {
        let tf = create_temp_json();
        let p = tf.path();
        let c = Config::from_file(p).unwrap();

        let want: Vec<&Tool> = c.tools.0.iter().filter(|t| t.manager == "arch").collect();
        assert_eq!(want, c.tools.filter_by_manager("arch"));

        let want: Vec<&Tool> = vec![];
        assert_eq!(want, c.tools.filter_by_manager("misc"));

        let want: Vec<&Tool> = vec![];
        assert_eq!(want, c.tools.filter_by_manager("rust"));
    }
}
