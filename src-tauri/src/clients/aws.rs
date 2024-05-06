use s3::bucket::Bucket;
use s3::creds::Credentials;

enum ClientError{
    InitClientError(String)
}

async fn client(){
    let bucket_name = "foodenie-ml";
    let region = "us-east-2".parse().unwrap();
    
    match  Credentials::default(){
        Ok(creds)=>{

            let bucket = Bucket::new(&bucket_name, region, creds).unwrap();

            let results = bucket.list("data/ml_state/base_meta".to_string(), None).await.unwrap();
            println!("Ok nice {:?}", results);

        }
        Err(err)=>println!("Error setting credentials: {:?}", err)
    }
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
pub async fn search_bucket(){
    // bucket_client().unwrap()
}

