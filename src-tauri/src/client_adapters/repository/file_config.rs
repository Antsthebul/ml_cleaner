use crate::client_adapters::models::{Configuration, ConfigurationFileError};
use std::{fs, path};

/// Returns the file and bool indicating if the file was created, or returns
/// error
pub fn create_file_if_not_present() -> Result<Configuration, ConfigurationFileError> {
    let file_name = "../ml_cleaner.conf";
    match path::Path::new(file_name).try_exists() {
        Ok(true) => {
            println!("{} configuration file exists", file_name);
            let file = Configuration::get_configuration_file().unwrap();
            Ok(file)
        }
        Ok(false) => match fs::File::create(file_name) {
            Ok(_) => {
                Configuration::update_configuration_file(Configuration {
                    default_machine: Some("".to_string()),
                    projects: Vec::new(),
                })?;
                println!("{} configuration file was created", file_name);
                let file = Configuration::get_configuration_file().unwrap();

                Ok(file)
            }
            Err(err) => Err(ConfigurationFileError(err.to_string())),
        },
        Err(err) => Err(ConfigurationFileError(err.to_string())),
    }
}
