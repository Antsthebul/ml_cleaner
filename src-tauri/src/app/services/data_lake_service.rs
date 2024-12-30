use crate::app::services::project_service;
use ml_cleaner::client_adapters::lake_client::*;
use bytes::Bytes;

use super::config_service;
pub struct LakeServiceError(String);

impl std::fmt::Display for LakeServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

async fn get_client_by_project(project_name: &str) -> Result<LakeClient, LakeServiceError> {
    let proj = config_service::get_project_by_project_name(project_name)
        .map_err(|err| LakeServiceError(err.to_string()))?;

    Ok(LakeClient::new("wack").map_err(|err| LakeServiceError(err.to_string()))?)
}
/// Thin wrapper for fetching image data for classes from repository utilizing project name along with
// desired class. To be used by other services
pub async fn get_data_for_class(
    project_name: &str,
    dep_name: &str,
    page: &str,
) -> Result<ImageListResult, LakeServiceError> {
    let proj = project_service::get_project_by_project_name(project_name)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    let mut path = String::from(""); // This is JUST  to get shit to pass
    path.push_str(dep_name);

    let lc = LakeClient::new("Fix this too")
        .map_err(|err| LakeServiceError(err.to_string()))?;

    Ok(lc
        .get_data_for_class(path.as_str(), page)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?)
}

// rename to get_dependent_data. This function should fetch classes
// along with traint/test data. Fetching process is done by first
// checking cache. If not in cache fetch from remote.

/// This will remove the object from the bucket, and remove the
/// file from the training data section.
pub async fn delete_data_for_class(
    project_name: &str,
    file_name: &str,
) -> Result<(), LakeServiceError> {
    let lc = get_client_by_project(project_name)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    lc.delete_object(file_name)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    Ok(())
}

/// Lists all classes found in data lake. Classes are folder/file names
pub async fn list_all_classes(project_name: &str) -> Result<Vec<String>, LakeServiceError> {
    let proj = project_service::get_project_by_project_name(project_name)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    // let lc = LakeClient::new(&proj.project.repository.name)
    //     .map_err(|err| LakeServiceError(err.to_string()))?;

    // Ok(lc
    //     .list_class_names(&proj.project.repository.path)
    //     .await
    //     .map_err(|err| LakeServiceError(err.to_string()))?)
    Ok(vec![])
}
/// Returns an images looked up within the specific project
pub async fn get_data_by_path(
    project_name: &str,
    file_path: &str,
) -> Result<ImageObject, LakeServiceError> {
    let lc = get_client_by_project(project_name)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    lc.get_data_for_image(file_path)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))
}

/// Thin file retreival wrapper to return dta as bytessdwe
pub async fn get_file(project_name: &str, file_path: &str) -> Result<Bytes, LakeServiceError> {
    let lc = get_client_by_project(project_name)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    Ok(lc
        .get_file(file_path)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?)
}

/// Generic wrapper to write files to associated data lake repository
pub async fn write_file(
    project_name: &str,
    file_path: &str,
    contents: &[u8],
) -> Result<(), LakeServiceError> {
    let lc = get_client_by_project(project_name)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    lc.write_file(file_path, contents)
        .await
        .map_err(|err| LakeServiceError(err.to_string()))?;

    Ok(())
}
