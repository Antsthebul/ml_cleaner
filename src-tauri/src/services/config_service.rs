use crate::common::response_types::{serialize_response, serialize_success};

/// Primaryily used to get all data for a configurations
/// if you want to fetch all projects, use `get_all_projects`
#[tauri::command]
pub async fn get_config()->Result<String, String>{
    let file = crate::services::get_configuration_file_for_commands()?;
    Ok(serialize_response("data".parse().unwrap(), file))
}

#[tauri::command]
pub async fn get_machine_from_deployment()->Result<String, String>{
    Ok(serialize_success("done".to_string()))
}