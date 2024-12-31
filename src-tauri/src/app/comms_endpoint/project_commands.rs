use crate::app::{
    common::response_types::{serialize_error, serialize_response},
    services::project_service::ProjectService,
};

#[tauri::command]
pub async fn get_all_projects() -> Result<String, String> {
    println!("[ProjectCommandEndpoint] Fetching All Projects route hit");
    let service = ProjectService::new().await
        .map_err(|err| serialize_error(err.to_string()))?;

    let projects = service.get_all_projects().await.map_err(|err|
        serialize_error(err.to_string() ))?;

    Ok(serialize_response("data".parse().unwrap(), projects))
}

#[tauri::command]
pub async fn get_project_by_project_name(project_name: &str) -> Result<String, String> {
    println!("[ProjectCommandEndpoint] Fetching project by name='{}'", project_name);
    
    let service = ProjectService::new().await
        .map_err(|err| serialize_error(err.to_string()))?;

    let project = service.get_project_by_name(project_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), project))
}

#[tauri::command]
pub async fn get_project_deployment(
    project_name: &str,
    deploy_name: &str,
) -> Result<String, String> {
    println!("[ProjectCommandEndpoint] Fetching project deplyment. deployment={}, project_name{}", deploy_name,project_name);
    
    let service = ProjectService::new().await
        .map_err(|err| serialize_error(err.to_string()))?;
    
    let deployment = service.get_project_deployment_by_name(project_name,deploy_name)
        .await
        .map_err(|err| serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(), deployment))
}
