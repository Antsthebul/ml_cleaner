use std::sync::Arc;

use crate::app::common::response_types::project_responses::{
    DeploymentResponse, ProjectResponse,
};
use ml_cleaner::client_adapters::{
    database::{ activity_log_db::ActivityLogDb, project_db::ProjectDb, AsyncDbClient, PGClient},
    models::{Configuration, Deployment, Project}};

use super::{config_service, data_lake_service};

#[derive(Debug)]
pub struct ProjectError(String);

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ProjectService{
    repo: ProjectDb,
    activity_repo: ActivityLogDb
}

impl ProjectService{
    pub async fn new() -> Result<Self,ProjectError>{
        let pg_client = PGClient::new()
            .await.map_err(|err| ProjectError(format!("project service could not be initialized. {}", err)))?;
        
        let client = Arc::new(pg_client);
        let repo = ProjectDb { client:client.clone() };
        let activity_repo = ActivityLogDb{client:client.clone()};
        

        Ok(ProjectService{repo, activity_repo})
        }
    
    
    pub async fn get_all_projects(&self) -> Result<Vec<Project>, ProjectError> {
        Ok(self.repo.get_all_projects()
        .await
        .map_err(|err|ProjectError(format!("project service failed to get all projects {}", err)))?
        )  
    }

    pub async fn get_project_by_name(&self, name:&str) -> Result<Project, ProjectError>{
        Ok(self.repo.get_project_by_name(name)
        .await
        .map_err(|err|ProjectError(format!("project service failed to get projects by name {}", err)))?
        )
    }
    
    
    /// Returns serialized Result or Error. The serialized result is
    /// a project with other additional metadata.
    pub async fn get_project_deployment_by_name(
        &self,
        project_name: &str,
        deploy_name: &str,
    ) -> Result<Deployment, ProjectError> {
        Ok(
            self.repo.get_project_deployment_by_name(project_name, deploy_name)
            .await
            .map_err(|err|
                ProjectError(format!(
                    "project service unable to retreive deployment using  
                    project={}, deployment={}. {}", project_name, deploy_name, err)))?
        )
    }
    
}
/// Returns a Project WITH dynamic attribute populates. This
/// is different from the config, which returns a bare config provided
/// attribute on a Project
pub async fn get_project_by_project_name(
    project_name: &str,
) -> Result<ProjectResponse<Project>, ProjectError> {
    let  project = config_service::get_project_by_project_name(project_name)
        .map_err(|err| ProjectError(err.to_string()))?;

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
