use std::sync::Arc;

use crate::app::common::response_types::project_responses::{DeploymentResponse, ProjectResponse};
use deadpool_postgres::Pool;
use ml_cleaner::client_adapters::{
    database::{activity_log_db::ActivityLogDb, project_db::ProjectDb, AsyncDbClient, PGClient},
    models::{Configuration, Deployment, Project},
};

use super::{config_service, data_lake_service};

#[derive(Debug)]
pub struct ProjectError(String);

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct ProjectService {
    repo: ProjectDb,
    activity_repo: ActivityLogDb,
}

impl ProjectService {
    pub async fn new(pool: Pool) -> Result<Self, ProjectError> {
        let conn1 = pool.get().await.map_err(|err| {
            ProjectError(format!("project service could not be initialized. {}", err))
        })?;

        let conn2 = pool.get().await.map_err(|err| {
            ProjectError(format!("project service could not be initialized. {}", err))
        })?;

        let repo = ProjectDb { client: conn1 };
        let activity_repo = ActivityLogDb { client: conn2 };

        Ok(ProjectService {
            repo,
            activity_repo,
        })
    }

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, ProjectError> {
        Ok(self.repo.get_all_projects().await.map_err(|err| {
            ProjectError(format!(
                "project service failed to get all projects {}",
                err
            ))
        })?)
    }

    pub async fn get_project_by_name(&self, name: &str) -> Result<Project, ProjectError> {
        Ok(self.repo.get_project_by_name(name).await.map_err(|err| {
            ProjectError(format!(
                "project service failed to get projects by name {}",
                err
            ))
        })?)
    }

    /// Returns serialized Result or Error. The serialized result is
    /// a project with other additional metadata.
    pub async fn get_project_deployment_by_name(
        &self,
        project_name: &str,
        deploy_name: &str,
    ) -> Result<Deployment, ProjectError> {
        Ok(self
            .repo
            .get_project_deployment_by_name(project_name, deploy_name)
            .await
            .map_err(|err| {
                ProjectError(format!(
                    "project service unable to retreive deployment using  
                    project={}, deployment={}. {}",
                    project_name, deploy_name, err
                ))
            })?)
    }

    pub async fn create_project(&self, new_project_name: &str) -> Result<Project, ProjectError> {
        self.repo
            .upsert_project(new_project_name)
            .await
            .map_err(|err| {
                ProjectError(format!(
                    "create project service could not create project. {err}"
                ))
            })?;

        let project = self
            .get_project_by_name(new_project_name)
            .await
            .map_err(|err| {
                ProjectError(format!(
                    "create project service failed to retreive new project. {err}"
                ))
            })?;

        self.activity_repo
            .create_activity_log("create".parse().unwrap(), "project was created")
            .await
            .map_err(|err| {
                ProjectError(format!(
                    "project service failed to create activiate log. {err}"
                ))
            })?;

        Ok(project)
    }

    pub async fn delete_deployment(
        &self,
        project_name: &str,
        deployment_name: &str,
    ) -> Result<(), ProjectError> {
        self.repo
            .delete_deployment(project_name, deployment_name)
            .await
            .map_err(|err| ProjectError(format!("delete deployment failed. {err}")))?;

        self.activity_repo
            .create_activity_log(
                "delete".parse().unwrap(),
                format!("{project_name} -> {deployment_name} was deleted").as_str(),
            )
            .await
            .map_err(|err| ProjectError(format!("failed to create activiity log, {err}")))?;
        Ok(())
    }
}
/// Returns a Project WITH dynamic attribute populates. This
/// is different from the config, which returns a bare config provided
/// attribute on a Project
pub async fn get_project_by_project_name(
    project_name: &str,
) -> Result<ProjectResponse<Project>, ProjectError> {
    let project = config_service::get_project_by_project_name(project_name)
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
