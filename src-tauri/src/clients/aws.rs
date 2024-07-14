use s3::bucket::Bucket;
use s3::creds::Credentials;
use serde::{Deserialize, Serialize};
use std::fmt::{self, write};

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

#[tauri::command]
pub async fn search_bucket()->Result<String, String>{
    let result =  bucket_client().unwrap()
        .get_object("data/ml_state/current/classes.txt").await;


    let data = result.unwrap();
    let headers =  data.headers();
    
    let full_text = String::from_utf8(data.into()).unwrap();
    
    let text_list = full_text.split("\n").collect::<Vec<&str>>();

    let response = serde_json::json!({
        "data":{
            "classes":text_list,
            "lastModified":headers["last-modified"]
        }
    });

    Ok(serde_json::to_string(&response).unwrap())
}

///Returns the list of depedent variables/classes the deployed
/// model was trained on as well as time the file as last modified
pub async fn get_classes_data(obj_path:&str)-> Result<ClassData,AWSClientError>{
    println!("Fetching classes from bucket");

    let data = bucket_client().unwrap()
        .get_object(obj_path.to_string()).await
        .map_err(|err| AWSClientError::ObjectRetrievalError(err.to_string()));
    
    match data {
        Ok(json_data)=>{
            let headers =  json_data.headers();
            let last_modified = headers.get("last-modified").unwrap().to_owned();
            let text_list =  String::from_utf8(json_data.into()).unwrap();
            Ok(ClassData{file_exists:true,last_modified, classes: text_list.split("\n").map(|v|v.to_string()).collect::<Vec<String>>()})
        },
        Err(_)=>{

            let headers = "".to_string().to_owned();
            
            Ok(ClassData{file_exists:false, last_modified:headers, classes:Vec::new()})
            }
        }
        
 }

async fn get_trained_model_info(){

}

async fn get_data_runs(){

}

async fn get_stage_data(){

}