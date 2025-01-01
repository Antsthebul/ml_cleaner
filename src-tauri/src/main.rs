// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cache_reg;
mod app;
mod daemon;
mod menu;

use tauri::Manager;
use ml_cleaner::client_adapters::{
    database::{build_conn_args, create_connection_pool, machine_db::MachineDb}, get_run_environment, model_hub::state_check_daemon
};
use postgres::Row;
use tokio::sync::Mutex;

use std::{env, path};

mod config;

use daemon::ModelHubRecord;


use app::comms_endpoint::{
    config_commands::get_config,
    data_lake_commands::get_data_for_class,
    image_verifier_commands::{
        get_class_names, get_unverified_images_for_class, keep_data_for_class, remove_image,
        sync_data,
    },
    model_hub_commands::{
        download_model, generate_test_train_data, get_machine_status, get_training_results,
        start_machine, stop_machine, stop_train_model, train_model,
    },
    project_commands::{
        create_project,
        get_all_projects, 
        get_project_by_project_name, 
        get_project_deployment,
        delete_deployment
    },
};
use dotenvy;

use deadpool_postgres::Pool;

pub struct AppState{
    pool: Pool
}

impl Default for AppState{
    fn default() -> Self {
        Self{
            pool: create_connection_pool(build_conn_args())
        }
    }
}


fn main() {
    let app_state = AppState::default();
    startup_function(&app_state.pool);

    tauri::Builder::default()
        .setup(|app|{
            app.manage(Mutex::new(app_state));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            //  Project commands
            create_project,
            get_all_projects,
            get_project_by_project_name,
            get_project_deployment,
            get_config,
            get_data_for_class,
            delete_deployment,
            remove_image,
            sync_data,
            // Image verifier commands
            get_class_names,
            get_unverified_images_for_class,
            keep_data_for_class,
            // Model Hub commands
            generate_test_train_data,
            train_model,
            stop_machine,
            start_machine,
            get_machine_status,
            stop_train_model,
            get_training_results,
            download_model // list_machines, get_machine_by_machine_id, get_machine_status,
                           // update_configuration_file_command, start_machine, stop_machine, create_new_project,
                           // get_all_projects, delete_project_by_name, train_model
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    }

fn load_env() -> Result<(), std::env::VarError> {
    dotenvy::from_path(path::Path::new("../.env"))
        .expect("env file shoud exist and have correct permissions");

    env::var("AWS_ACCESS_KEY")?;
    env::var("AWS_SECRET_KEY")?;
    env::var("PAPERSPACE_API_KEY")?;

    Ok(())
}

fn startup_function(pool: &Pool) {
    println!("Running Startup functions...");
    load_env().expect("should be failed to find requried env vars");
    let _ = cache_reg::create_cache();

    let records = tauri::async_runtime::block_on(async move {
        let machine_db = MachineDb{client: pool.get().await.unwrap()};

        let mut results = machine_db.get_all_machines()
            .await;
        
        match results{
            Ok(machines)=>machines,
            Err(err)=> {println!("Unable to sync machines. {err}");
                        vec![]}
        }

    });

    if records.len() == 0 {
        println!("All machines sync'd at this time")
    }
    for r in records {
        println!("[Startup] Syncing state of DB with API...\n");
        tauri::async_runtime::spawn(async move {
            state_check_daemon(r.provider, r.machine_id, String::from("startup")).await;
        });
    }

    println!("Startup process complete!\n");
    println!("Running in **{}**", get_run_environment())
}
