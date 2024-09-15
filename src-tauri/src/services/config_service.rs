use crate::common::response_types::{serialize_response, serialize_success};
use app::file_config::{Configuration, Project};

#[derive(Debug)]
pub struct ConfigSerivceError(String);

impl std::fmt::Display for ConfigSerivceError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.0)
    }
}
/// Primarily used to get all data for a configurations
/// if you want to fetch all projects, use `get_all_projects`.
/// This is the dispatch location, so if databasebackend is used
/// this  "should" call the backend
pub fn get_config()->Result<Configuration, ConfigSerivceError>{
crate::services::get_configuration_file_for_commands()
    .map_err(|err|ConfigSerivceError(err.to_string()))
}

pub fn get_project_by_project_name(project_name:&str) -> Result<Project, ConfigSerivceError>{
    Ok(Configuration::get_project_by_project_name(project_name)
        .map_err(|err|ConfigSerivceError(err.to_string()))?)    
}