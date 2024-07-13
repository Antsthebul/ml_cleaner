// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utilities;
mod clients;
mod config;
mod commands;

use config::create_file_if_not_present;
use commands::{get_config, update_configuration_file_command, create_new_project, get_all_projects, get_config_by_project_name};
use clients::{
  aws::search_bucket,
  paperspace::{list_machines,get_status, is_running, start_machine, stop_machine}
};
fn main() {

  // Startup functions
  if let Err(_err) = crate::utilities::load_env(){
    println!("AWS ACCESS/SECRET KEYS must exist in .env file")
  }


  if let Err(err) = create_file_if_not_present(){
    println!("Unable to create file due to {}", err)
  }

  
  
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    search_bucket, list_machines, get_status, is_running, 
    get_config,update_configuration_file_command, start_machine, stop_machine, create_new_project,
    get_all_projects, get_config_by_project_name
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
