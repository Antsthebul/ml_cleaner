use std::fmt;

use crate::services::data_lake_service::{list_all_classes, get_data_for_class};
use app::components::adapters::image_verifier::{ImageVerifierClient, ImageVerifierError};

#[derive(Debug)]
pub struct ImageStoreServiceError(String);

impl fmt::Display for ImageStoreServiceError{
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}
/// Retrieves all classes(folder names) from bucket and checks
/// to see if they exist in database, if not insert as
/// unverified record
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
            let mut class_data = get_data_for_class(project_name, &c, "").await
            .map_err(|err|ImageStoreServiceError(err.to_string()))?;
            
            let mut images = class_data.images;
            let mut has_next_page = true;
            let mut page = String::from("");
            loop{

                println!("Loop start -> {}", images.len());
                for im in &images{
                    verifier_client.insert_image_verification(&c, &im.file_path, false).await
                    .map_err(|err|ImageStoreServiceError(err.to_string()))?;
                }

                println!("Successfully inserted '{}' image(s) to verified_images", images.len());

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
        };
        
    }
        Ok(())
}