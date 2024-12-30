pub mod config_service;
pub mod data_lake_service;
pub mod image_verifier_service;
pub mod model_hub_service;

pub mod project_service;

use ml_cleaner::client_adapters::models::Configuration;

pub struct ServiceError(String);

impl std::fmt::Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn get_configuration_file_for_commands() -> Result<Configuration, ServiceError> {
    Configuration::get_configuration_file().map_err(|err| {
        ServiceError(format!(
            "Failed to retrive config file in service. {:?}",
            err
        ))
    })
}
