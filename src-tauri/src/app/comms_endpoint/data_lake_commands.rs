use crate::app::{common::response_types::{serialize_error, serialize_response, serialize_success},
    services::data_lake_service
};

#[tauri::command]
pub async fn get_data_for_class(
    project_name: &str,
    dep_name: &str,
    page: &str,
) -> Result<String, String> {
    println!(
        "AWS Command GET Data for Class - proj: {}, class[dep]:{}, page:{}\n",
        project_name, dep_name, page
    );
    let images = data_lake_service::get_data_for_class(project_name, dep_name, page)
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_response("data".parse().unwrap(), images))
}

#[tauri::command]
pub async fn delete_data_for_class(project_name: &str, file_path: &str) -> Result<String, String> {
    println!(
        "Delete command. Removing {} from {} in aws",
        project_name, file_path
    );
    // delete path
    data_lake_service::delete_data_for_class(project_name, file_path)
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}
