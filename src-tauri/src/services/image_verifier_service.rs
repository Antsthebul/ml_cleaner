use std::fmt;

use crate::services::data_lake_service::{list_all_classes, get_data_for_class, delete_data_for_class};
use app::components::adapters::image_verifier::{ImageVerifiedRecord, ImageVerifierClient, ImageVerifierError, Pagination};
use serde::{Deserialize, Serialize};

use super::data_lake_service;

#[derive(Debug)]
pub struct ImageStoreServiceError(String);

#[derive(Serialize, Deserialize)]
pub struct ImageCollection{
    images:Vec<ImageMeta>,
    previous_page:Option<String>,
    next_page:Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct ImageMeta{
    pub b64:String,
    pub file_path: String
}

impl fmt::Display for ImageStoreServiceError{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}
/// Retrieves all classes(folder names) from bucket and checks
/// to see if they exist in database, if not insert as
/// unverified record in the verified_iamges table
pub async fn sync(project_name:&str)->Result<(), ImageStoreServiceError>{
    let mut verifier_client = ImageVerifierClient::new().await.unwrap();
    
    for c in list_all_classes(project_name).await
        .map_err(|err|ImageStoreServiceError(err.to_string()))?{
        
        let class_id = match verifier_client.get_class_id_by_name(&c).await{
            Ok(v)=>Some(v),
            Err(ImageVerifierError::ObjectNotFound)=>{println!("'{}' not found in database", c);None},
            Err(v)=>{
                return Err(ImageStoreServiceError(v.to_string()))
            }
        };

        if class_id.is_none(){

            let _ = verifier_client.insert_new_class(&c).await
                .map_err(|err|ImageStoreServiceError(err.to_string()))?;
            println!("Added '{}' to class table", c);
        }else{
            println!("'{}' exists", c);
        }
        println!("Syncing data..\n");

        let mut class_data = get_data_for_class(project_name, &c, "").await
            .map_err(|err|ImageStoreServiceError(err.to_string()))?;
        
        let mut images = class_data.images;
        let mut has_next_page = true;
        let mut page = String::from("");
        
        loop{
            println!("Loop start -> {}", images.len());
            let mut dup_count = 0;
            let mut inserted = 0;
            for im in &images{
                match verifier_client.insert_image_verification(&c, &im.file_path, false).await{
                    Ok(_)=>inserted+=1,
                    Err(ImageVerifierError::DuplicateEntry)=>{
                        dup_count +=1;
                        println!("Duplicate entry found. Continuing")},
                    Err(err)=>{return Err(ImageStoreServiceError(err.to_string()))}
                };
            };
            println!("Successfully inserted '{}' image(s) to verified_images. Found '{}' duplicates", inserted, dup_count);

            match class_data.next_page{
                None=>has_next_page=false,
                Some(p)=>page=p
            }

            if images.len() == 0 || !has_next_page{
                println!("Exiting loop");
                break
            }
            class_data = get_data_for_class(project_name, &c, &page).await
            .map_err(|err|ImageStoreServiceError(err.to_string()))?;
            
            images = class_data.images;

            println!("End of loop");
        };        
    }
        Ok(())
}

/// Get all classes for project
/// TODO: Except project name
pub async fn get_all_classes() -> Result<Vec<String>, ImageStoreServiceError>{
    let mut verifier_client = ImageVerifierClient::new().await.unwrap();
    Ok( verifier_client.get_all_class_names().await
        .map_err(|err|ImageStoreServiceError(err.to_string()))?)

}

pub async fn get_unverified_images_for_class(project_name:&str, class_name:&str, page:Option<&str>)->Result<ImageCollection, ImageStoreServiceError>{
    let mut verifier_client = ImageVerifierClient::new().await.unwrap();

    let image_paths = verifier_client.get_unverified_images_for_class(class_name, page).await
        .map_err(|err|ImageStoreServiceError(err.to_string()))?;

    let mut results: Vec<ImageMeta> = vec![];

    for p in image_paths.0{
        let tmp = data_lake_service::get_data_by_path(project_name, &p).await
            .map_err(|err|ImageStoreServiceError(err.to_string()))?;
        
        results.push(ImageMeta{
            b64:tmp.b64,
            file_path:p
        })
    };
    let paginator = image_paths.1;
    Ok(ImageCollection { images: results, previous_page:paginator.previous_page, next_page:paginator.next_page })
}

pub async fn verify_image(file_path:&str)-> Result<(), ImageStoreServiceError>{
    let mut verifier_client = ImageVerifierClient::new().await.unwrap();

    verifier_client.update_image_verification_status(file_path, true).await
        .map_err(|err|ImageStoreServiceError(err.to_string()))?;
    
    Ok(())
}

/// Removes images from the database AND from the repository.
pub async fn remove_image(project_name:&str, file_path:&str) -> Result<(), ImageStoreServiceError>{
    let mut verifier_client = ImageVerifierClient::new().await.unwrap();

    verifier_client.delete_image(file_path).await
        .map_err(|err|ImageStoreServiceError(err.to_string()))?;
    
    delete_data_for_class(project_name, file_path).await
        .map_err(|err|ImageStoreServiceError(err.to_string()))?;

    Ok(())
}