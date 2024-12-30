use std::collections::HashMap;

use crate::{clients::aws::get_classes_data, config::{Configuration, Project}, utilities::{serialize_error, serialize_success}};





#[tauri::command]
pub async fn update_configuration_file_command(file:&str)->Result<String, String>{
    println!("Updating configuration file");
    match  serde_json::from_str::<Configuration>(file){
        Ok(config)=>{
            if let Err(err)= Configuration::update_configuration_file(config){
                return Ok(serialize_error(format!("Unable to save file: {:#?}", err)))
            };
            Ok(serialize_success("success"))
        },
        Err(err)=>Err(serialize_error(err))
        }   
}

#[tauri::command]
pub async fn create_new_project(project:&str) -> Result<String, String>{
    let mut config = Configuration::get_configuration_file()
        .map_err(|err| serialize_error(err))?;

    match serde_json::from_str::<Project>(project){
        Ok(project)=>{
            let _ = config.add_project(project);
            if let Err(err) = Configuration::update_configuration_file(config){
                return Err(serialize_error(err))
                
            };
            Ok(serialize_success("ok"))
        },
        Err(err)=>Err(serialize_error(err))
    }
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