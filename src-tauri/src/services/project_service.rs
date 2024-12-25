use crate::common::response_types::project_responses::{
    DeploymentResponse, ProjectResponse,
};
use app::file_config::{Configuration, FileAttr, Project};

use super::{config_service, data_lake_service};

#[derive(Debug)]
pub struct ProjectError(String);

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub async fn get_all_projects() -> Result<Vec<Project>, ProjectError> {
    let config = Configuration::get_all_projects().map_err(|err| ProjectError(err.to_string()))?;

    Ok(config.iter().map(|c| c.to_owned()).collect())
}

/// Returns serialized Result or Error. The serialized result is
/// a project with other additional metadata.
pub async fn get_project_deployment(
    project_name: &str,
    deploy_name: &str,
) -> Result<DeploymentResponse, ProjectError> {
    let project = config_service::get_project_by_project_name(project_name)
        .map_err(|err| ProjectError(err.to_string()))?;

    let deployment = project.get_project_deployment(deploy_name).unwrap();

    // We want to either get the file(s) or generate them
    // for now fail fast and return single errors, not lists
    if let Some(files) = deployment.files {
        for f in vec!["train", "test"] {
            if let None = files.get(f) {
                return Err(ProjectError(String::from(format!("No {:?} file found", f))));
            }
        }
    };
    let dr = DeploymentResponse {
        name: deployment.name,
        machines: deployment.machines,
        files: None,
    };

    Ok(dr)
}

/// Returns a Project WITH dynamic attribute populates. This
/// is different from the config, which returns a bare config provided
/// attribute on a Project
pub async fn get_project_by_project_name(
    project_name: &str,
) -> Result<ProjectResponse<Project>, ProjectError> {
    let mut project = config_service::get_project_by_project_name(project_name)
        .map_err(|err| ProjectError(err.to_string()))?;

    let base_train_data_path = "data/base/test.json";
    let base_test_data_path = "data/base/test.json";

    // Check if files exists
    let train_exists = check_if_file_exists(project_name, &base_train_data_path).await?;
    let test_exists = check_if_file_exists(project_name, &base_test_data_path).await?;

    project.train_file = Some(FileAttr {
        path: base_train_data_path.to_owned(),
        exists: train_exists,
    });
    project.test_file = Some(FileAttr {
        path: base_test_data_path.to_owned(),
        exists: test_exists,
    });

    let pr = ProjectResponse { project };
    Ok(pr)
}

/// Checks if file exists in repository. If an error is encounture,
/// returns error
pub async fn check_if_file_exists(
    project_name: &str,
    file_path: &str,
) -> Result<bool, ProjectError> {
    match data_lake_service::get_file(project_name, &file_path).await {
        Err(err) => {
            let err_string = err.to_string();
            if !err_string.contains("does not exist") {
                return Err(ProjectError(err_string));
            } else {
                Ok(false)
            }
        }
        Ok(_) => Ok(true),
    }
}
