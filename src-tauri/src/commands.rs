use std::collections::HashMap;

use crate::{clients::aws::get_classes_data, config::{Configuration, Project}, utilities::{serialize_error, serialize_success}};
use toml;

fn get_configuration_file_for_commands() -> Result<String, String>{
    match Configuration::get_configuration_file(){
        Ok(config)=>{

            let success_response = serde_json::json!({"data":{"configuration":config}});   
            Ok(serde_json::to_string(&success_response).unwrap())
        }
        Err(err)=> Ok(serde_json::to_string(err.message.as_str()).unwrap())
    }
    }
#[tauri::command]
pub async fn get_config()->Result<String, String>{
    let file = get_configuration_file_for_commands()?;
    Ok(file)
}

#[tauri::command]
pub async fn update_configuration_file_command(file:&str)->Result<String, String>{
    println!("Updating configuration file");
    match  serde_json::from_str::<Configuration>(file){
        Ok(config)=>{
            if let Err(err)= Configuration::update_configuration_file(config){
                return Ok(serde_json::to_string(&serde_json::json!({"error":format!("Unable to save file: {:#?}", err)})).unwrap())
            };
            Ok(serde_json::to_string(&serde_json::json!({"data":"success"})).unwrap())
        },
        Err(err)=>Err(serialize_error(err))
        }   
}

#[tauri::command]
pub async fn create_new_project(project:&str) -> Result<String, String>{
    let mut config = Configuration::get_configuration_file()
        .map_err(|err| serde_json::to_string(&serde_json::json!({"error":err.to_string().as_str()})).unwrap())?;

    match serde_json::from_str::<Project>(project){
        Ok(project)=>{
            let _ = config.add_project(project);
            if let Err(err) = Configuration::update_configuration_file(config){
                return Err(serde_json::to_string(&serde_json::json!({"error":err.to_string().as_str()})).unwrap())
                
            };
            Ok(serde_json::to_string(&serde_json::json!({"data":"ok"})).unwrap())
        },
        Err(err)=>Err(serde_json::to_string(&serde_json::json!({"error":err.to_string().as_str()})).unwrap())
    }
}

#[tauri::command]
pub async fn get_all_projects()->Result<String, String>{
    let config = Configuration::get_all_projects()
        .map_err(|err|serde_json::to_string(&serde_json::json!({"error":err.to_string()})).unwrap())?;
    
    let projects:Vec<&Project> = config.values().collect();
    Ok(serde_json::to_string(&serde_json::json!({"data":projects})).unwrap())
}

#[tauri::command]
pub async fn get_project_by_project_name(name:&str)-> Result<String, String>{
    let project = Configuration::get_project_by_project_name(name)
        .map_err(|err|serialize_error(err.to_string()))?;
    
    let file_path = match &project.classes_file{
        Some(file) => file,
        None=>{return Ok(serialize_success(serde_json::json!({"project":project, "class_data":HashMap::<String, String>::new()})))}
    };

    let class_data = get_classes_data(file_path).await
    .map_err(|err|serialize_error(err.to_string()))?;

    let return_data = serde_json::json!(
        {"data":
            {"project":project,
            "class_data":class_data
        }});
    Ok(serde_json::to_string(&return_data).unwrap())
}

#[tauri::command]
pub async fn delete_project_by_name(name:&str)-> Result<String, String>{
    let mut config = Configuration::get_configuration_file()
    .map_err(|err| serialize_error(err))?;

    config.projects.remove(name);

    Configuration::update_configuration_file(config)
    .map_err(|err|serialize_error(err))?;

    Ok(serialize_success("success"))

}