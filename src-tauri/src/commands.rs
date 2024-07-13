use crate::config::{Configuration, Project};
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

    match  toml::from_str::<Configuration>(file){
        Ok(config)=>{
            if let Err(err)= Configuration::update_configuration_file(config){
                return Ok(serde_json::to_string(&serde_json::json!({"error":format!("Unable to save file: {:#?}", err)})).unwrap())
            };
            Ok(serde_json::to_string(&serde_json::json!({"data":"success"})).unwrap())
        },
        Err(err)=>Ok(serde_json::to_string(&serde_json::json!({"error":err.to_string().as_str()})).unwrap())
        }   
}

#[tauri::command]
pub async fn create_new_project(project:&str) -> Result<String, String>{
    let mut config = Configuration::get_configuration_file()
        .map_err(|err|  serde_json::to_string(&serde_json::json!({"error":err.to_string().as_str()})).unwrap())?;

    match serde_json::from_str::<Project>(project){
        Ok(project)=>{
            config.add_project(project);
            if let Err(err) = Configuration::update_configuration_file(config){
                return Err(serde_json::to_string(&serde_json::json!({"error":err.to_string().as_str()})).unwrap())
                
            };
            Ok(serde_json::to_string(&serde_json::json!({"data":"ok"})).unwrap())
        },
        Err(err)=>Err(serde_json::to_string(&serde_json::json!({"error":err.to_string().as_str()})).unwrap())
    }
}
