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
pub struct ImageObject{
    b64: String,
    file_name:String,
}

#[derive(Deserialize, Serialize)]
pub struct ClassData{
    file_exists:bool,
    classes: Vec<String>,
    last_modified: String
}

#[derive(Debug)]
pub enum AWSClientError{
    InitClientError(String),
    ObjectRetrievalError(String)
}

impl  fmt::Display for AWSClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            AWSClientError::InitClientError(s) => write!(f, "{}",s),
            AWSClientError::ObjectRetrievalError(s)=> write!(f, "{}", s)
        }
    }
}

fn bucket_client(bucket_name:&str)-> Result<Bucket, AWSClientError>{
    let region = "us-east-2".parse().unwrap();

    match Credentials::default(){
        Ok(creds)=>Ok(*Bucket::new(&bucket_name, region, creds).unwrap()),
        Err(err)=>Err(AWSClientError::InitClientError(format!("Error setting credentials: {:?}", err)))
    }
}


/// Returns the list of depedent variables/classes the deployed
/// model was trained on as well as time the file as last modified
// pub async fn get_classes_data(file_path: &str)-> Result<ClassData,AWSClientError>{
//     println!("Fetching classes from bucket");
//     // Read text from [repo]
//     let data = bucket_client().unwrap()
//         .get_object(file_path).await
//         .map_err(|err| AWSClientError::ObjectRetrievalError(err.to_string()));
    
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

/// Returns a paginated list of 'in-storage' image URLs. Lookups performed by complete path
/// ie. 'data/images/banangas', delimiter="" . Which means get everything in this path
pub async fn get_data_for_class(bucket_name:&str, path:&str)->Result<Vec<ImageObject>, AWSClientError>{
    
    let res = bucket_client(bucket_name).unwrap().list(path.to_owned(),Some("".to_string())).await
        .map_err(|err| AWSClientError::ObjectRetrievalError(err.to_string()))?;

    // Not sure why this is returning a list?
    let contents = &res[0].contents;

    let mut res = Vec::<ImageObject>::new();
    for ix in 0..5{
        let c = &contents[ix];
        let data = bucket_client(bucket_name).unwrap().get_object(&c.key).await
        .map_err(|err| AWSClientError::ObjectRetrievalError(err.to_string()))?;

        res.push(
            ImageObject{
                b64:BASE64_STANDARD.encode(data.bytes().to_vec()),
                file_name:c.key.to_owned(),
            }
        );

    }
    Ok(res)
} 

// Delete actual object from bucket
// pub async fn delete_object(file_name:&str)->Result<(), AWSClientError>{
//     match bucket_client().unwrap().delete_object(file_name.to_string()).await{
//         Ok(_)=>Ok(()),
//         Err(err)=>Err(AWSClientError::ObjectRetrievalError(err.to_string()))
//     }
// }
// PUT item in bucket
// pub async fn update_object(file_path:&str, content:&[u8])->Result<(), AWSClientError>{
//     match bucket_client().unwrap().put_object(file_path.to_string(), content).await{
//         Ok(_)=>Ok(()),
//         Err(err)=>Err(AWSClientError::ObjectRetrievalError(err.to_string()))
//     }
        
// } 

/// Generic get_object function
// pub async fn get_object_by_file_path(file_path: &str)->Result<(), AWSClientError>{
//     match bucket_client().unwrap().get_object(file_path.to_string()).await{
//         Ok(_)=>Ok(()),
//         Err(err)=>Err(AWSClientError::ObjectRetrievalError(err.to_string()))
//     }
// }

///Create a cache file to prevent large reads of databases or
/// aggregating folder names
pub async fn sync_repo(bucket_name:&str, base_path:&str)->Result<Vec<u8>, AWSClientError>{
    let file_name = ".sync";
    
    match get_sync_file(bucket_name, file_name).await{
        Ok(rd) => {
            let tmp = rd.bytes().to_owned().to_vec();
            return Ok(tmp)
        
        },
        Err(err)=>{
            // Actually an XML string, may be better to use XML Parser
            if  !err.to_string().contains("HTTP 404"){
                return Err(AWSClientError::ObjectRetrievalError(err.to_string()))
            }
            // implement "sync" logic here...

            let res = bucket_client(bucket_name).unwrap().list(base_path.to_owned(), Some("/".to_string())).await.unwrap();
            
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
            create_sync_file(bucket_name, file_name, classes.as_bytes()).await?;
            return Ok(classes.as_bytes().to_vec())
        }
    }
}

pub async fn get_sync_file(bucket_name:&str,file_name:&str) -> Result<ResponseData,AWSClientError>{

    bucket_client(bucket_name).unwrap().get_object(file_name).await
        .map_err(|err|AWSClientError::ObjectRetrievalError(err.to_string()))
}

pub async fn create_sync_file(bucket_name:&str, file_name:&str, text_lines:&[u8]) -> Result<(), AWSClientError>{
   println!("Creating sync file");
    bucket_client(bucket_name).unwrap().put_object(file_name, text_lines).await
    .map_err(|err| AWSClientError::ObjectRetrievalError(err.to_string()))?;
    
    println!("Sync file created..\n");

    Ok(())
}