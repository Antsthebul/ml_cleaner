use crate::{
    common::response_types::{serialize_error, serialize_response, serialize_success},
    services::image_verifier_service::sync};


#[tauri::command]
pub async fn sync_data(project_name:&str)-> Result<String, String>{  
    sync(project_name).await
        .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_success("OK"))
}