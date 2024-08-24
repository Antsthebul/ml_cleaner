/// Services comminucate with the library to pull in
/// data
pub mod project_service;
pub mod config_service;

use app::file_config::Configuration;
use crate::common::response_types::serialize_error;

pub fn get_configuration_file_for_commands() -> Result<Configuration, String>{
    Configuration::get_configuration_file().map_err(|err|{
        serialize_error(err)
    })
}