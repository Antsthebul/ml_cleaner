use app::components::adapters::aws;
use crate::services::project_service;

pub struct AWSServiceError(String);

impl std::fmt::Display for AWSServiceError{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}", self.0)
    }
}

/// Thin wrapper for fetching classes data from repository utilizing project name along with
// desired class. To be used by other services
pub async fn get_data_for_class(project_name:&str, dep_name:&str)-> Result<Vec<aws::ImageObject>, AWSServiceError>{
    let proj = project_service::get_project_by_project_name(project_name).await
        .map_err(|err|AWSServiceError(err.to_string()))?;

    let mut path = proj.project.repository.path;
    path.push_str(dep_name);

    Ok(aws::get_data_for_class(&proj.project.repository.name, path.as_str() ).await
    .map_err(|err|AWSServiceError(err.to_string()))?)


 // rename to get_dependent_data. This function should fetch classes
 // along with traint/test data. Fetching process is done by first 
 // checking cache. If not in cache fetch from remote.
}

/// This will remove the object from the bucket, and remove the
/// file from the training data section.
fn fix_this(){}
// pub async fn delete_data_for_class(file_name:&str)-> Result<(), aws::AWSClientError>{
    
//     // delet epath
//     aws::delete_object(file_name).await?;
//     Ok(())
// }