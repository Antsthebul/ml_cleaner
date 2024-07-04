use s3::bucket::Bucket;
use s3::creds::Credentials;

#[derive(Debug)]
enum ClientError{
    InitClientError(String)
}

fn bucket_client()-> Result<Bucket, ClientError>{
    let bucket_name = "foodenie-ml";
    let region = "us-east-2".parse().unwrap();

    match Credentials::default(){
        Ok(creds)=>Ok(Bucket::new(&bucket_name, region, creds).unwrap()),
        Err(err)=>Err(ClientError::InitClientError(format!("Error setting credentials: {:?}", err)))
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

async fn get_trained_classes_data(){

}

async fn get_trained_model_info(){

}

async fn get_data_runs(){

}

async fn get_stage_data(){

}