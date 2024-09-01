use crate::{
    common::response_types::{serialize_error, serialize_response},
    services::config_service
};

#[tauri::command]
pub async fn get_config()->Result<String, String>{
    let file = config_service::get_config().await
        .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_response("data".parse().unwrap(), file))
}