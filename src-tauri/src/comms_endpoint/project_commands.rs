use crate::{
    common::response_types::{serialize_error, serialize_response},
    services::project_service,
};

#[tauri::command]
pub async fn get_all_projects() -> Result<String, String> {
    let projects = project_service::get_all_projects().await.map_err(|err| {
        serialize_error(format!(
            "[Project Command - GEt all project failed]: {:?}",
            err
        ))
    })?;

    Ok(serialize_response("data".parse().unwrap(), projects))
}

#[tauri::command]
pub async fn get_project_by_project_name(project_name: &str) -> Result<String, String> {
    println!("[CommandEndpoint] Fetching project '{}'", project_name);
    let project = project_service::get_project_by_project_name(project_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), project))
}

#[tauri::command]
pub async fn get_project_deployment(
    project_name: &str,
    deploy_name: &str,
) -> Result<String, String> {
    let deployment = project_service::get_project_deployment(project_name, deploy_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), deployment))
}
