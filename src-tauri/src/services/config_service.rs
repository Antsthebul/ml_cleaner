use crate::common::response_types::{serialize_response, serialize_success};
use app::file_config::Configuration;

#[derive(Debug)]
pub struct ConfigSerivceError(String);

impl std::fmt::Display for ConfigSerivceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.0)
    }
}
/// Primaryily used to get all data for a configurations
/// if you want to fetch all projects, use `get_all_projects`
pub async fn get_config()->Result<Configuration, ConfigSerivceError>{
crate::services::get_configuration_file_for_commands()
    .map_err(|err|ConfigSerivceError(err.to_string()))
}

#[tauri::command]
pub async fn get_machine_from_deployment()->Result<String, String>{
    Ok(serialize_success("done".to_string()))
}