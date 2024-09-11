use crate::{
    common::response_types::{serialize_error, serialize_response, serialize_success},
    services::image_verifier_service
};

#[tauri::command]
pub async fn sync_data(project_name:&str)-> Result<String, String>{  
    image_verifier_service::sync(project_name).await
        .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_success("OK"))
}

#[tauri::command]
pub async fn get_class_names(project_name:&str) -> Result<String, String>{
    let res = image_verifier_service::get_all_classes().await
        .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_response("data".parse().unwrap(), res))

}

#[tauri::command]
pub async fn get_unverified_images_for_class(project_name:&str, class_name:&str, page:&str) ->Result<String, String>{
    println!("Get unverified images for class '{}' in project '{}' page '{}'", project_name, class_name, page);
    let mut paginator = None;
    if !page.is_empty(){
        paginator = Some(page);
    };
    let res = image_verifier_service::get_unverified_images_for_class(project_name, class_name, paginator).await
     .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_response("data".parse().unwrap(), res))   
}

#[tauri::command]
pub async fn keep_data_for_class(project_name:&str, file_path:&str)->Result<String, String>{
    println!("Keep command. Keeping '{}' for '{}' ", file_path, project_name);

    image_verifier_service::verify_image(file_path).await
        .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn remove_image(project_name:&str, file_path:&str) -> Result<String, String>{
    println!("Removing Image command. '{}' for project '{}'",file_path, project_name);

    image_verifier_service::remove_image(project_name, file_path).await
        .map_err(|err|serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}