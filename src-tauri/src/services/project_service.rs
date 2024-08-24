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
#[tauri::command]
pub async fn get_project_environment(project_name:&str, env_name:&str) -> Result<String, String>{
    let project = Configuration::get_project_by_project_name(project_name)
    .map_err(|err|serialize_error(err))?;

    let environment = project.get_project_environment(env_name).unwrap();

    let file_path = match &environment.classes_file{
        Some(file) => file,
        // Return bare 'inititalized' 
        None=>{
            let res = serde_json::json!({"project":project});
            return Err(serialize_response("data".parse().unwrap(), res))}
    };

    // let class_data = get_classes_data(file_path).await
    // .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_response("data".parse().unwrap(), environment))
}
#[tauri::command]
pub async fn get_project_by_project_name(project_name:&str)-> Result<String, String>{
    
    let project = Configuration::get_project_by_project_name(project_name)
        .map_err(|err|serialize_error(err))?;

    Ok(serialize_response("data".parse().unwrap(),project))
}
