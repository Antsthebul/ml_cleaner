// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod common;
mod services;

use std::{
  env, 
  path, 
};

mod config;
// mod repository;
use dotenvy;
use crate::services::{
  project_service::{get_all_projects,get_project_environment, get_project_by_project_name},
  config_service::get_config,
};
use app::file_config;
// use app::clients::{aws::get_classes_data, file_config};

// use commands::{get_config, update_configuration_file_command, create_new_project, get_all_projects, get_project_by_project_name,
//    delete_project_by_name};
// use app::{
//   aws::search_bucket,
//   paperspace::{list_machines,get_machine_by_machine_id, get_machine_status, start_machine, stop_machine, train_model}
// };
fn main() {

  // Startup functions
  if let Err(_err) = load_env(){
    println!("AWS ACCESS/SECRET KEYS must exist in .env file")
  }


  if let Err(err) = file_config::create_file_if_not_present(){
    println!("Unable to create file due to {}", err)
  }

  
  
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    get_all_projects, get_config, get_project_by_project_name,
    get_project_environment,
    // list_machines, get_machine_by_machine_id, get_machine_status, 
    // update_configuration_file_command, start_machine, stop_machine, create_new_project,
    // get_all_projects, delete_project_by_name, train_model
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


fn load_env() -> Result<(), std::env::VarError>{
    
  dotenvy::from_path(path::Path::new("../.env"))
  .expect("env file shoud exist and have correct permissions");

  env::var("AWS_ACCESS_KEY")?;
  env::var("AWS_SECRET_KEY")?;
  env::var("PAPERSPACE_API_KEY")?;
  Ok(())
}