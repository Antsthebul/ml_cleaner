use app::components::adapters::aws;
use crate::common::response_types::{serialize_response,serialize_error};

#[tauri::command]
pub async fn get_data_for_class(dep_name:&str)-> Result<String, String>{
    let images = aws::get_data_for_class(dep_name, "some-path").await
    .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_response("data".parse().unwrap(),images))
}