use app::{file_config::{Project,Configuration},
    get_classes_data
    };
use crate::common::response_types::{ serialize_error, serialize_response};
use std::collections::HashMap;

#[tauri::command]
pub async fn get_all_projects()->Result<String, String>{
    let config = Configuration::get_all_projects()
        .map_err(|err|serialize_error(err))?;
    
    let projects:Vec<&Project> = config.iter().collect();
    Ok(serialize_response("data".parse().unwrap(), projects))
}

/// Returns serialized Result or Error. The serialized result is
/// a project with other additional metadata.
#[tauri::command]
pub async fn get_project_deployment(project_name:&str, deploy_name:&str) -> Result<String, String>{
    let project = Configuration::get_project_by_project_name(project_name)
    .map_err(|err|serialize_error(err))?;

    let deployment = project.get_project_deployment(deploy_name).unwrap();

    let file_path = match &deployment.classes_file{
        Some(file) => file,
        // Return bare 'inititalized' 
        None=>{
            let res = serde_json::json!({"deployment":deployment, "classes_data":Vec::<String>::new()});
            return Err(serialize_response("data".parse().unwrap(), res))}
    };
    // Add meta data. No need for 'response' struct
    let class_data = get_classes_data(file_path).await
    .map_err(|err|serialize_error(err.to_string()))?;
    let response = serde_json::json!({"deployment":deployment, "classes_data":class_data});

    Ok(serialize_response("data".parse().unwrap(), response))
}
#[tauri::command]
pub async fn get_project_by_project_name(project_name:&str)-> Result<String, String>{
    
    let project = Configuration::get_project_by_project_name(project_name)
        .map_err(|err|serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(),project))
}
