// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


mod common;
mod comms_endpoint;
mod services;
mod cache_reg;
mod daemon;
mod menu;

use std::{ env, path};

mod config;
use app::{create_client, file_config::create_file_if_not_present, state_check_daemon};
use daemon::{gather_existing_machines_in_config};
// mod repository;
use dotenvy;
use menu::build_menu;
use crate::comms_endpoint::{
  project_commands::{get_all_projects, get_project_by_project_name, get_project_deployment},
  config_commands::get_config,
  data_lake_commands::get_data_for_class,
  image_verifier_commands::{sync_data, get_class_names, get_unverified_images_for_class, keep_data_for_class, remove_image},
  model_hub_commands::{generate_test_train_data, train_model}
};



fn main() {
  startup_function();  
  
  tauri::Builder::default()
  .menu(build_menu())
  .invoke_handler(tauri::generate_handler![
    //  Project commands
    get_all_projects, get_project_by_project_name, get_project_deployment, 
    get_config, get_data_for_class, remove_image,sync_data,
    // Image verifier commands
    get_class_names,get_unverified_images_for_class, keep_data_for_class,
    // Model Hub commands
    generate_test_train_data, train_model
    // list_machines, get_machine_by_machine_id, get_machine_status, 
    // update_configuration_file_command, start_machine, stop_machine, create_new_project,
    // get_all_projects, delete_project_by_name, train_model
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
  println!("never makes its");
}


fn load_env() -> Result<(), std::env::VarError>{
    
  dotenvy::from_path(path::Path::new("../.env"))
  .expect("env file shoud exist and have correct permissions");

  env::var("AWS_ACCESS_KEY")?;
  env::var("AWS_SECRET_KEY")?;
  env::var("PAPERSPACE_API_KEY")?;
  Ok(())
}

fn startup_function(){
  load_env().expect("should be failed to find requried env vars");
  let _ = cache_reg::create_cache();
  let config = create_file_if_not_present().unwrap();
  let records = gather_existing_machines_in_config(&config);
  if records.len() > 0 {
    println!("Record got damn greater")
  }else{
    println!("0 recs homie")
  }

  for r in records{
    println!("first record");
    tauri::async_runtime::spawn(async move{
      state_check_daemon(r.provider,r.machine_id).await;
    });
  };
 
}