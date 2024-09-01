
use crate::common::response_types::{serialize_success,serialize_response,serialize_error};
use crate::services::aws_service;

// #[tauri::command]
// pub async fn get_data_for_class(dep_name:&str)-> Result<String, String>{
//     let images = aws_service::get_data_for_class(dep_name).await
//     .map_err(|err| serialize_error(err.to_string()))?;

//     Ok(serialize_response("data".parse().unwrap(),images))
// }

// #[tauri::command]
// pub async fn delete_data_for_class(file_name:&str)-> Result<String, String>{
    
//     // delete path
//     aws_service::delete_data_for_class(file_name).await
//         .map_err(|err| serialize_error(err.to_string()))?;
    



//     Ok(serialize_success("success"))
// }