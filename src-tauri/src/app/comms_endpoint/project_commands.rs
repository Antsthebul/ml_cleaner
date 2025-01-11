use tauri::State;

use crate::{
    app::{
        common::response_types::{serialize_error, serialize_response},
        services::project_service::ProjectService,
    },
    AppState,
};

#[tauri::command]
pub async fn get_all_projects(state: State<'_, AppState>) -> Result<String, String> {
    println!("[ProjectCommandEndpoint] Fetching All Projects route hit");

    let service = ProjectService::new(state.pool.clone())
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    let projects = service
        .get_all_projects()
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_response("data".parse().unwrap(), projects))
}

#[tauri::command]
pub async fn get_project_by_project_name(
    state: State<'_, AppState>,
    project_name: &str,
) -> Result<String, String> {
    println!(
        "[ProjectCommandEndpoint] Fetching project by name='{}'",
        project_name
    );

    let service = ProjectService::new(state.pool.clone())
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    let project = service
        .get_project_by_name(project_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), project))
}

#[tauri::command]
pub async fn get_project_deployment(
    state: State<'_, AppState>,
    project_name: &str,
    deploy_name: &str,
) -> Result<String, String> {
    println!(
        "[ProjectCommandEndpoint] Fetching project deplyment. deployment={}, project_name{}",
        deploy_name, project_name
    );

    let service = ProjectService::new(state.pool.clone())
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    let deployment = service
        .get_project_deployment_by_name(project_name, deploy_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), deployment))
}

#[tauri::command]
pub async fn create_project(
    state: State<'_, AppState>,
    new_project_name: &str,
) -> Result<String, String> {
    println!("[ProjectCommandEndpoint] Creating project name={new_project_name}");

    let service = ProjectService::new(state.pool.clone())
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    let project = service
        .create_project(new_project_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), project))
}
#[tauri::command]
pub async fn delete_deployment(
    state: State<'_, AppState>,
    project_name: &str,
    deployment_name: &str,
) -> Result<String, String> {
    println!(
        "[ProjectCommandEndpoint] Deleting deployment from
        project deployment={deployment_name}, project={project_name}"
    );

    let service = ProjectService::new(state.pool.clone())
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    let project = service
        .delete_deployment(project_name, deployment_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), project))
}
