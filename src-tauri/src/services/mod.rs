/// Services comminucate with the library to pull in
/// data
pub mod project_service;
pub mod config_service;
pub mod aws_service;

use app::file_config::Configuration;

pub struct ServiceError(String);

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.0)
    }
}

pub fn get_configuration_file_for_commands() -> Result<Configuration, ServiceError>{
    Configuration::get_configuration_file().map_err(|err|{
        ServiceError(format!("Failed to retrive config file in service. {:?}", err))
    })
}