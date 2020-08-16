use clap::{App, AppSettings, Arg};

pub mod stm;

fn main() {
    let config = stm::Config::default().expect("error while loading config");
    std::env::set_var("STM_CONFIG_PATH", stm::app_dir());

    let valid_managers = config.managers.names();

    let has_valid_manager = move |v: String| -> Result<(), String> {
        if !&valid_managers.contains(&v) {
            return Err(format!("invalid manager {}", v));
        }
        Ok(())
    };

    let matches = App::new("System Tool Manager")
        .about("System Tool Manager (STM) is a tool for install and updates any system tools in a easy way.")
        .author("Emerson Max de Medeiros Silva <emersonmx@gmail.com>")
        .version("1.0.0")
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            App::new("install").about("Run managers install").arg(
                Arg::with_name("managers")
                    .help("the managers to run install")
                    .index(1)
                    .required(true)
                    .multiple(true)
                    .validator(has_valid_manager.clone())
            ),
        )
        .subcommand(
            App::new("update").about("Run managers update").arg(
                Arg::with_name("managers")
                    .help("the managers to run update")
                    .index(1)
                    .required(true)
                    .multiple(true)
                    .validator(has_valid_manager.clone())
            ),
        )
        .subcommand(App::new("list").about("List all available managers"))
        .get_matches();

    match matches.subcommand() {
        ("install", Some(install_matches)) => {
            let args: Vec<String> = install_matches
                .values_of("managers")
                .unwrap()
                .map(|m| m.to_string())
                .collect();
            install_command(&config, args);
        }
        ("update", Some(update_matches)) => {
            let args: Vec<String> = update_matches
                .values_of("managers")
                .unwrap()
                .map(|m| m.to_string())
                .collect();
            update_command(&config, args);
        }
        ("list", Some(_)) => {
            list_command(&config);
        }
        _ => {}
    }
}

fn install_command(config: &stm::Config, managers: Vec<String>) {
    managers
        .iter()
        .map(|m| config.find_manager(&m).unwrap())
        .for_each(|m| {
            let packages: Vec<&str> = config
                .tools
                .filter_by_manager(&m.name)
                .into_iter()
                .filter(|t| !t.is_installed())
                .map(|t| t.package.as_str())
                .collect();
            m.install_packages(packages)
                .expect("failed to execute process");
        });
}

fn update_command(config: &stm::Config, managers: Vec<String>) {
    managers
        .iter()
        .map(|m| config.find_manager(&m).unwrap())
        .for_each(|m| {
            let packages: Vec<&str> = config
                .tools
                .filter_by_manager(&m.name)
                .into_iter()
                .map(|t| t.package.as_str())
                .collect();
            m.update_packages(packages)
                .expect("failed to execute process");
        });
}

fn list_command(config: &stm::Config) {
    config
        .managers
        .names()
        .iter()
        .for_each(|m| println!("{}", m));
}
