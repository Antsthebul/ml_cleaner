// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dotenv;
mod utilities;


fn main() {
  dotenv::dotenv().ok();
  if let Err(_err) = crate::utilities::load_env(){
    println!("AWS ACCESS/SECRET KEYS must exist in .env file")
  }

  tauri::Builder::default()
  // .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
