// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod utilities;
mod clients;
mod config;

use config::{get_config, update_default_machine};
use clients::{
  aws::search_bucket,
  paperspace::{list_machines,get_status, is_running, start_machine, stop_machine}
};
fn main() {

  if let Err(_err) = crate::utilities::load_env(){
    println!("AWS ACCESS/SECRET KEYS must exist in .env file")
  }
  
  
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
    search_bucket, list_machines, get_status, is_running, 
    get_config, update_default_machine, start_machine, stop_machine
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
