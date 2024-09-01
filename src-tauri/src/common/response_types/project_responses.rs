use app::components::repository::file_config::ProjectMachine;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]

pub struct FileDataResponse{
    pub path:String,
    pub last_modified: String
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ProjectResponse<T>{
    pub project:T,
    pub classes: Vec<String>
}

#[derive(Debug, Deserialize, Serialize)]
pub struct DeploymentResponse {
    pub files: Option<HashMap<String, String>>,
    pub name: String,
    pub machines: Vec<ProjectMachine>,

}