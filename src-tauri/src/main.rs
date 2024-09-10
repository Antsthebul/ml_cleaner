// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 'mod' defs here apply to the entire crate;
// doesn't mean this must be used here 
mod common;
mod comms_endpoint;
mod services;

use std::{
  collections::HashMap, env, fs, io::Write, path 
};

mod config;
// mod repository;
use dotenvy;
use crate::comms_endpoint::{
  project_commands::{get_all_projects, get_project_by_project_name, get_project_deployment},
  config_commands::get_config,
  data_lake_commands::get_data_for_class,
  image_verifier_commands::{sync_data, get_class_names, get_unverified_images_for_class, keep_data_for_class, remove_image}

};

use app::file_config;
use std::sync::mpsc::{channel, Sender, Receiver};

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
  let rt = tokio::runtime::Runtime::new().unwrap();
  
  // let mut project_class_map = HashMap::new();

  // Create local cache
  if !path::Path::new(".cache/").exists(){
    fs::create_dir(".cache/").expect("Cannot create local dir")
  }

  // match file_config::create_file_if_not_present(){
  //   Ok(config)=>{
  //     rt.block_on(async{

  //       for proj in config.projects{
  //         // create a way to prevent manual sync
  //         // AND make this non blocking

  //         if let Ok(class_list) = sync_repo(&proj.repository.name, proj.repository.path.as_str()).await{
  //           let proj_name = &proj.name;
  //           project_class_map.insert(proj_name.to_owned(), class_list);
  //         } else{
  //           println!("Failed");
  //         };
  
  //       }
  //     })
  //   }
  //   Err(err)=>{println!("Unable to create file due to {}", err)}
  // }
  // Easier to call local than remote for data
  // let json_contents = serde_json::json!(project_class_map);
  // fs::write(".cache/map.json", json_contents.to_string()).expect("failed to write data");
  // we STILL would need to have the db be in syndcd with the actual
  // iamges
  // generate train/test json files if they dont exist for each deployment
  // save in remote

  
  
  tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![
     get_all_projects, get_config, get_project_by_project_name,
    get_project_deployment, get_data_for_class, remove_image,sync_data,
    get_class_names,get_unverified_images_for_class, keep_data_for_class
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