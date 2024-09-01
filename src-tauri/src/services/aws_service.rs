use app::components::adapters::aws;

// Thin wrapper for fetching classes data from aws
// tobe used by other services
// pub async fn get_data_for_class(dep_name:&str)-> Result<Vec<aws::ImageObject>, aws::AWSClientError>{
//  aws::get_data_for_class(dep_name, "some-path").await

//  // rename to get_dependent_data. This function should fetch classes
//  // along with traint/test data. Fetching process is done by first 
//  // checking cache. If not in cache fetch from remote.
// }

/// This will remove the object from the bucket, and remove the
/// file from the training data section.
fn fix_this(){}
// pub async fn delete_data_for_class(file_name:&str)-> Result<(), aws::AWSClientError>{
    
//     // delet epath
//     aws::delete_object(file_name).await?;
//     Ok(())
// }