/// AWS client crate to facilitate
/// fetching and saving objects

use s3::{bucket::Bucket, request::ResponseData};
use s3::creds::Credentials;
use serde::{Deserialize, Serialize};
use std::fs;
use std::{fmt::{self, write}};
use crate::file_config::Configuration;
use bytes::Bytes;
use base64::prelude::*;
use regex::Regex;

#[derive(Deserialize, Serialize)]

pub struct ImageListResult{
    pub previous_page:Option<String>,
    pub images: Vec<ImageObject>,
    pub next_page:Option<String>
}

#[derive(Deserialize, Serialize)]
pub struct ImageObject{
    pub b64: String,
    pub file_path:String,
}

#[derive(Deserialize, Serialize)]
pub struct ClassData{
    file_exists:bool,
    classes: Vec<String>,
    last_modified: String
}

#[derive(Debug)]
pub enum LakeClientError{
    InitClientError(String),
    ObjectRetrievalError(String)
}

pub struct LakeClient{
    client: Bucket
}

impl LakeClient{

    /// Returns a ready-to-use client in the given region.
    pub fn new(bucket_name:&str)-> Result<LakeClient, LakeClientError>{
        let region = "us-east-2".parse().unwrap();
    
        match Credentials::default(){
            Ok(creds)=>Ok(LakeClient{ client:*Bucket::new(&bucket_name, region, creds).unwrap() }),
            Err(err)=>Err(LakeClientError::InitClientError(format!("Error setting credentials: {:?}", err)))
        }
        
    }
    /// Returns a paginated list of 'in-storage' image URLs. Lookups performed by complete path
    /// ie. 'data/images/banangas', delimiter="" . Which means get everything in this path
    pub async fn get_data_for_class(self, path:&str, page:&str)->Result<ImageListResult, LakeClientError>{
        let mut continuation_token:Option<_> = None;

        if !page.is_empty(){
            continuation_token = Some(page.to_owned());
        };

        let res = self.client.list_page(path.to_owned(),Some("".to_string()), continuation_token.clone(), None, Some(10)).await
            .map_err(|err| LakeClientError::ObjectRetrievalError(err.to_string()))?;

        // Not sure why this is returning a list?
        let contents = &res.0.contents;

        let mut images = Vec::<ImageObject>::new();
        for c in contents{

            let data = self.client.get_object(&c.key).await
            .map_err(|err| LakeClientError::ObjectRetrievalError(err.to_string()))?;

            images.push(
                ImageObject{
                    b64:BASE64_STANDARD.encode(data.bytes().to_vec()),
                    file_path:c.key.to_owned(),
                }
            );

        }

        Ok(    ImageListResult{
            previous_page:res.0.continuation_token,
            images:images,
            next_page:res.0.next_continuation_token
        })
    } 

    /// Delete actual object from bucket. 
    pub async fn delete_object(self, file_path:&str)->Result<(), LakeClientError>{
        match self.client.delete_object(file_path.to_string()).await{
            Ok(_)=>Ok(()),
            Err(err)=>Err(LakeClientError::ObjectRetrievalError(err.to_string()))
        }
    }    

    /// Create a cache file to prevent large reads of databases or
    /// aggregating folder names. Data is saved as bytes
    pub async fn sync_repo(self, base_path:&str)->Result<Vec<u8>, LakeClientError>{
        let file_name = ".sync";
        
        match &self.get_sync_file(file_name).await{
            Ok(rd) => {
                let tmp = rd.bytes().to_owned().to_vec();
                return Ok(tmp)
            
            },
            Err(err)=>{
                // Actually an XML string, may be better to use XML Parser
                if  !err.to_string().contains("HTTP 404"){
                    return Err(LakeClientError::ObjectRetrievalError(err.to_string()))
                }
                // implement "sync" logic here...

                let res = &self.client.list(base_path.to_owned(), Some("/".to_string())).await.unwrap();
                
                let mut classes : String = String::from("");
                let file_name_re = Regex::new(r"\/0?\d+_").unwrap();
                for c in res[0].common_prefixes.as_ref().unwrap(){
                    let prefix = &mut c.prefix.to_owned();
                    if !file_name_re.is_match(prefix){
                        let raw_name = prefix.strip_prefix(base_path).unwrap();
                        let name = raw_name.strip_suffix("/").unwrap();
                        let mut tmp = String::from(name.to_owned());
                        tmp.push_str("\n");
                        classes.push_str(tmp.as_str());
                    }
                }
                self.create_sync_file(file_name, classes.as_bytes()).await?;
                return Ok(classes.as_bytes().to_vec())
            }
        }
    }

    /// Lists the folder/file names of bucket and strips off base_path
    pub async fn list_class_names(self, base_path:&str)->Result<Vec<String>, LakeClientError>{
        let mut classes:Vec<String> = vec![];
        let res = &self.client.list(base_path.to_owned(), Some("/".to_string())).await.unwrap();
        let file_name_re = Regex::new(r"\/0?\d+_").unwrap();

        for c in res[0].common_prefixes.as_ref().unwrap(){
            let prefix = &mut c.prefix.to_owned();
            if !file_name_re.is_match(prefix){
                let raw_name = prefix.strip_prefix(base_path).unwrap();
                let name = raw_name.strip_suffix("/").unwrap();
                let tmp = String::from(name.to_owned());
                classes.push(tmp);
            }
        };

        Ok(classes)

    }

    pub async fn get_sync_file(&self, file_name:&str) -> Result<ResponseData,LakeClientError>{

        self.client.get_object(file_name).await
            .map_err(|err|LakeClientError::ObjectRetrievalError(err.to_string()))
    }

    pub async fn create_sync_file(self, file_name:&str, text_lines:&[u8]) -> Result<(), LakeClientError>{
        println!("Creating sync file");
        self.client.put_object(file_name, text_lines).await
            .map_err(|err| LakeClientError::ObjectRetrievalError(err.to_string()))?;
        
        println!("Sync file created..\n");

        Ok(())
    }    

    pub async fn get_data_for_image(self, file_name:&str) -> Result<ImageObject, LakeClientError>{
        let res = self.client.get_object(file_name).await
            .map_err(|err| LakeClientError::ObjectRetrievalError(err.to_string()))?;
        
        Ok(ImageObject{
            b64:BASE64_STANDARD.encode(res.bytes().to_vec()),
            file_path:file_name.to_owned(),
        })
  
    }
    /// Generic file retreival method
    pub async fn get_file(self, file_path:&str)->Result<Bytes, LakeClientError>{
        println!("[Log] Getting file {}", file_path);
        let res = self.client.get_object(file_path).await
            .map_err(|err| LakeClientError::ObjectRetrievalError(err.to_string()))?;

        Ok(res.bytes().clone())
    }

    pub async fn write_file(self, file_path:&str, content:&[u8]) -> Result<(), LakeClientError>{
        let res = self.client.put_object(file_path, content).await
            .map_err(|err| LakeClientError::ObjectRetrievalError(err.to_string()))?;
    
        println!("Saved to location {}", res);
        println!(" {:?}", res);
        Ok(())
    }
}

impl  fmt::Display for LakeClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            LakeClientError::InitClientError(s) => write!(f, "{}",s),
            LakeClientError::ObjectRetrievalError(s)=> write!(f, "{}", s)
        }
    }
}




/// Returns the list of depedent variables/classes the deployed
/// model was trained on as well as time the file as last modified
// pub async fn get_classes_data(file_path: &str)-> Result<ClassData,LakeClientError>{
//     println!("Fetching classes from bucket");
//     // Read text from [repo]
//     let data = bucket_client().unwrap()
//         .get_object(file_path).await
//         .map_err(|err| LakeClientError::ObjectRetrievalError(err.to_string()));
    
//     match data {
//         Ok(json_data)=>{
//             let headers =  json_data.headers();
//             // println!("{:?}", headers);
//             let last_modified =match headers.get("last-modified"){
//                 Some(val)=>val,
//                 None=>""
//             };
//             let text_list =  String::from_utf8(json_data.into()).unwrap();
//             Ok(ClassData{file_exists:true,last_modified:last_modified.to_owned(), classes: text_list.split("\n").map(|v|v.to_string()).collect::<Vec<String>>()})
//         },
//         Err(_)=>{

//             let headers = "".to_string().to_owned();
            
//             Ok(ClassData{file_exists:false, last_modified:headers, classes:Vec::new()})
//             }
//         }
        
//  }

 /// Get meta information saved on model such as
 /// training time, epcohs, layers, etc...
async fn get_trained_model_info(){

}

/// Fetches files, saved on config for the data runs directory
async fn get_data_runs(){

}

/// Fetches file saved on config if exists, from AWS bucket
/// returns dependent variales that are staged to be deployed
async fn get_stage_data(){

}



// PUT item in bucket
// pub async fn update_object(file_path:&str, content:&[u8])->Result<(), LakeClientError>{
//     match bucket_client().unwrap().put_object(file_path.to_string(), content).await{
//         Ok(_)=>Ok(()),
//         Err(err)=>Err(LakeClientError::ObjectRetrievalError(err.to_string()))
//     }
        
// } 

// Generic get_object function
// pub async fn get_object_by_file_path(file_path: &str)->Result<(), LakeClientError>{
//     match bucket_client().unwrap().get_object(file_path.to_string()).await{
//         Ok(_)=>Ok(()),
//         Err(err)=>Err(LakeClientError::ObjectRetrievalError(err.to_string()))
//     }
// }
