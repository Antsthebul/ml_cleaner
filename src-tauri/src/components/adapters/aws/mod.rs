/// AWS client crate to facilitate
/// fetching and saving objects

use s3::bucket::Bucket;
use s3::creds::Credentials;
use serde::{Deserialize, Serialize};
use std::{fmt::{self, write}};
use crate::file_config::Configuration;
use bytes::Bytes;
use base64::prelude::*;

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

fn bucket_client()-> Result<Bucket, AWSClientError>{
    let bucket_name = "foodenie-ml";
    let region = "us-east-2".parse().unwrap();

    match Credentials::default(){
        Ok(creds)=>Ok(Bucket::new(&bucket_name, region, creds).unwrap()),
        Err(err)=>Err(AWSClientError::InitClientError(format!("Error setting credentials: {:?}", err)))
    }
}


/// Returns the list of depedent variables/classes the deployed
/// model was trained on as well as time the file as last modified
pub async fn get_classes_data(file_path: &str)-> Result<ClassData,AWSClientError>{
    println!("Fetching classes from bucket");
    // Read text from [repo]
    let data = bucket_client().unwrap()
        .get_object(file_path).await
        .map_err(|err| AWSClientError::ObjectRetrievalError(err.to_string()));
    
    match data {
        Ok(json_data)=>{
            let headers =  json_data.headers();
            // println!("{:?}", headers);
            let last_modified =match headers.get("last-modified"){
                Some(val)=>val,
                None=>""
            };
            let text_list =  String::from_utf8(json_data.into()).unwrap();
            Ok(ClassData{file_exists:true,last_modified:last_modified.to_owned(), classes: text_list.split("\n").map(|v|v.to_string()).collect::<Vec<String>>()})
        },
        Err(_)=>{

            let headers = "".to_string().to_owned();
            
            Ok(ClassData{file_exists:false, last_modified:headers, classes:Vec::new()})
            }
        }
        
 }

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

/// Returns a paginated list of 'in-storage' image URLs related
/// to the dependent variable ie. 'apples'
pub async fn get_data_for_class(dep_name: &str, storage_path:&str)->Result<Vec<ImageObject>, AWSClientError>{
    let mut path = "data/images/".to_string();
    path.push_str(dep_name);
    
    let res = bucket_client().unwrap().list(path,Some("".to_string())).await
        .map_err(|err| AWSClientError::ObjectRetrievalError(err.to_string()))?;

    // Not sure why this is returning a list?
    let contents = &res[0].contents;

    let mut res = Vec::<ImageObject>::new();
    for ix in 0..5{
        let c = &contents[ix];
        let data = bucket_client().unwrap().get_object(&c.key).await
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