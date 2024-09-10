// This module is to acts as a tracking/sync logic for
// verifiying images.

use core::fmt;
// use postgres::{Client, NoTls};
use tokio_postgres::{NoTls, Error, Client};

#[derive(Debug)]
pub enum ImageVerifierError{
    ClientConfigurationError(String),
    ClientRetreivalError(String),
    ObjectNotFound,
    MultipleRowsFound,
    DuplicateEntry,
    QueryFailed
}

pub struct ImageVerifierClient {
    client: Client
}

pub struct ImageVerifiedRecord{
    pub class_id:i32,
    pub verified:bool,
    pub file_path:String
}

impl ImageVerifierClient{
    pub async fn new() -> Result<ImageVerifierClient, ImageVerifierError>{
        let (c, conn) = tokio_postgres::connect("host=host.docker.internal user=ml_cleaner password=ml_cleaner dbname=local_db", NoTls).await
        .map_err(|err| ImageVerifierError::ClientConfigurationError(err.to_string()))?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Connection err: {}", e)
            }
        });

        Ok(ImageVerifierClient { client: c })

    }

    /// Insert new class into classes table. Returns class PK
    pub async fn insert_new_class(&mut self, class_name:&str)->Result<i32,ImageVerifierError>{
       
        let new_row = self.client.query("INSERT INTO classes (name) VALUES($1) RETURNING id", &[&class_name]).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;
    
        let row = new_row.first().unwrap();
        Ok(row.get("id"))
    }
    /// Retrive class_id from classes table
    pub async fn get_class_id_by_name(&mut self, class_name:&str)-> Result<i32, ImageVerifierError>{
    
        let new_row = self.client.query("SELECT id FROM classes where name=$1", &[&class_name]).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;
    
        match new_row.first(){
            Some(row) =>Ok(row.get("id")),
            None=> Err(ImageVerifierError::ObjectNotFound)
        }
            
    }
    
    pub async fn insert_image_verification(&mut self,class_name:&str,file_path:&str, status:bool)->Result<(), ImageVerifierError>{
        let class_id = self.get_class_id_by_name(class_name).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;
    
        self.client.execute("INSERT INTO verified_images (class_id, file_path, verified) VALUES ($1, $2, $3)",&[&class_id, &file_path, &status]).await
            .map_err(|err|{
                let val = err.to_string();
                if val.contains("duplicate key"){
                    ImageVerifierError::DuplicateEntry
                }else{
                    ImageVerifierError::QueryFailed
                }
            })?;
    
        Ok(())
    
    }
    pub async fn get_image_verification_by_file_path(&mut self, file_path:&str)->Result<ImageVerifiedRecord, ImageVerifierError>{
        let row = self.client.query("SELECT class_id, file_path, verified FROM verified_image where file_path=$1", &[&file_path])
            .await.map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;

        if row.len() > 1{
            return Err(ImageVerifierError::MultipleRowsFound);
        };

        let data = row.first().unwrap();

        Ok(ImageVerifiedRecord{class_id:data.get(0), verified:data.get(2), file_path:data.get(1)})

    }

    pub async fn get_all_class_names(&mut self)->Result<Vec<String>, ImageVerifierError>{
        let rows = self.client.query("SELECT name FROM classes", &[]).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;
        
        
        Ok(rows.iter().map(|r|r.get::<usize, String>(0)).collect())
    }
    /// Will only return file_paths. This is the file_path related to the
    /// ''data lake/repository
    pub async fn get_unverified_images_for_class(&mut self, class_name:&str) -> Result<Vec<String>, ImageVerifierError>{
        let class_id = self.get_class_id_by_name(class_name).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;

        let rows = self.client.query("SELECT file_path FROM verified_images WHERE verified=false and class_id=$1 LIMIT 10", &[&class_id]).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;
        
        Ok(rows.iter().map(|r|r.get::<usize, String>(0)).collect())

    }

    pub async fn is_image_path_verified(&mut self, file_path:&str)->Result<bool, ImageVerifierError>{
        let rows = self.client.query("SELECT verified FROM verified_images where file_path=$1", &[&file_path]).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;

        let row = rows.first();
        if row.is_none(){
            Err(ImageVerifierError::ObjectNotFound)
        }else{
            let data = row.unwrap();
            let res:bool = data.get(0);
            Ok(res)
        }
    }

    pub async fn update_image_verification_status(&mut self, file_path:&str, status:bool)-> Result<(), ImageVerifierError>{
        self.client.query("UPDATE verified_images set verified=$1 where file_path=$2", &[&status, &file_path]).await
            .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;
        Ok(())
    }

    pub async fn delete_image(&mut self, file_path:&str) -> Result<(), ImageVerifierError>{
        self.client.query("DELETE from verified_images where file_path=$1",&[&file_path]).await
        .map_err(|err| ImageVerifierError::ClientRetreivalError(err.to_string()))?;
        Ok(())        
    }

}

impl fmt::Display for ImageVerifierError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        match self{

            Self::ClientConfigurationError(s)| Self::ClientRetreivalError(s) =>write!(f, "{}", s),
            Self::ObjectNotFound=>write!(f, "Object not found"),
            Self::MultipleRowsFound=> write!(f, "MultipleRows found"),
            Self::DuplicateEntry=>write!(f, "Entry exists"),
            Self::QueryFailed=>write!(f, "Query Unsuccessful")

        }
    }
}


