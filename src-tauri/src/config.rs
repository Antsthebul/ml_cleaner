use std::{fs, path, io::{self, prelude::*}};
use toml;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Config {
    configuration:Configuration
}

#[derive(Deserialize, Serialize)]
struct Configuration{
    default_machine: Option<String>
}

impl Config {
    pub fn get_or_create() -> Self{
        let file_name = "../ml_cleaner.conf";
        let file = match path::Path::new(file_name).try_exists(){
            Ok(true)=>fs::File::open(file_name).unwrap(),
            Ok(false)=>fs::File::create(file_name).unwrap(),
            Err(err)=>panic!("Cannot open file {:?}", err)
        };

        let mut buf_reader = io::BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let configuration:Configuration = toml::from_str(&contents).unwrap();
        
        Config{configuration}
    }
    pub fn update_machine_default(self, machine_id:&str){
        let value = match machine_id{

            "resetDefaultMachine"=>"default_machine =\"\"".to_string(),
            y=>format!("default_machine=\"{}\"", y)
        };
        fs::write("../ml_cleaner.conf",value.as_bytes()).unwrap();
    }
}

#[tauri::command]
pub async fn get_config()->Result<String, String>{
    let config = Config::get_or_create();

    let response = serde_json::json!({"data":config});

    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn update_default_machine(machine_id:&str)->Result<String, String>{
    let config = Config::get_or_create();

    config.update_machine_default(machine_id);
    let response = serde_json::json!({
        "data":"success"
    });
    Ok(serde_json::to_string(&response).unwrap())
}