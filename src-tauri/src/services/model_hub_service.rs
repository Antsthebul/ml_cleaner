use std::{fmt, collections::HashMap};
use app::create_client;
use serde_json::json;
use crate::cache_reg::update_cache;

use super::{ 
    config_service, 
    data_lake_service, image_verifier_service, project_service::get_project_by_project_name};
use rand::prelude::SliceRandom;

use app::Client;

pub struct ModelHubServiceError(String);

impl fmt::Display for ModelHubServiceError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}
/// Generates test/train data for related project.
/// For now we generate from scratch
pub async fn generate_test_train_data(project_name:&str, train_pct:Option<f64>) -> Result <(), ModelHubServiceError>{
    let pr = get_project_by_project_name(project_name).await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;
    
    let image_map = image_verifier_service::list_all_images_by_class().await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    
    let all_images = shuffle_images(&image_map);

    let mut pct = 0.8;

    if let Some(val) = train_pct{
        pct = val
    };

    let split_data = split_dataset(&all_images, pct);
    
    write_dataset_to_repo(project_name, split_data).await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    println!("training {} images", all_images.len());

    Ok(())
}

fn shuffle_images(images:&HashMap<String, Vec<String>>)->Vec<(String, String)>{
    let mut all_images: Vec<(String, String)>= Vec::new();
    for  set_ in images.into_iter(){
        let c_name = set_.0;
        for im in set_.1{
            all_images.push((c_name.to_owned(), im.to_owned()));
        }
    };

    let mut rng = rand::thread_rng();
    all_images.shuffle(&mut rng);
    all_images
}

/// Splits the dataset base on the training data percentage amount (`train_pct`) provided
fn split_dataset(data:&Vec<(String, String)>, train_pct:f64)-> HashMap<String, &[(String, String)]>{
    let len_of_data = data.len() as f64;
    let limit = (len_of_data * train_pct) as usize;

    let train = &data[0..limit];
    let test = &data[limit..data.len()];

    let mut hm = HashMap::new();
    hm.insert("train".to_owned(), train);
    hm.insert("test".to_owned(), test);
    hm


}
/// Writes the test/train data, provided as a HashMap, to the data lake repository
async fn write_dataset_to_repo(project_name:&str, data: HashMap<String, &[(String, String)]>) -> Result<(), ModelHubServiceError>{
    for data_type in vec!["train", "test"]{

        let data = data.get(data_type).unwrap();

        let file_path = format!("data/base/{}.json", data_type);
        
        let train_contents = serde_json::to_vec(&json!({data_type:data})).unwrap();
        
        data_lake_service::write_file(project_name, &file_path, train_contents.as_slice()).await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;
        
        println!("Successfully wrote to {}", file_path);
    }

    Ok(())
}
/// Retreives status of a machine by looking up the related model hub
/// vendor, and accessing their API.
pub async fn get_machine_status(project_name:&str,machine_id:&str)-> (){
    // let pc = PaperSpaceClient::new();

    // let machine = pc.get_machine_status(project_name, machine_id).await
    //     .map_err(|err|serialize_error(err.to_string()))?;

    // let response = serde_json::json!({"data":machine});
    // Ok(serde_json::to_string(&response).unwrap())
    ()
}

// TODO: We might not want 'list' machines, in the way that the UI
// allows. List machines works by API key whereas Orkestr8ML works by 
// project.
pub async fn list_machines(){
    // let pc = PaperSpaceClient::new();
    // let machines = pc.get_machines().await
    // .map_err(|err|serialize_error(err.to_string()))?;

    // let response = serde_json::json!({"data":machines});
    // Ok(serde_json::to_string(&response).unwrap())
}

/// Spins up a machine in the related provider environment.
pub async fn start_machine(machine_id:&str){
    // let pc = PaperSpaceClient::new();
    // let  _ = pc.handle_machine_run_state(machine_id, "start").await
    //     .map_err(|err|serde_json::to_string(&serde_json::json!({"error":err.to_string()})).unwrap())?;

    // let response = serde_json::json!({"data":"success"});

    // Ok(serde_json::to_string(&response).unwrap())
}



pub async fn stop_machine(machine_id:&str){
    // let pc = PaperSpaceClient::new();
    // pc.handle_machine_run_state(machine_id, "stop").await
    //     .map_err(|err|serialize_error(err))?;

    // Ok(serialize_success("success"))
}


pub async fn train_model(deployment_name:&str, project_name:&str, machine_id:&str) -> Result<(), ModelHubServiceError>{
    // We use the 'minimal' retreival method, since we only need provider
    let proj = config_service::get_project_by_project_name(project_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;
    let dep = proj.get_project_deployment(deployment_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;
    
    let mach = dep.get_machine_by_machine_id(machine_id)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();
    
    
    let ip_address = match mach.ip_addr{
        None=>{
            let res = mdl_hub_client.handle_machine_run_state(machine_id, "start").await
                .map_err(|err| ModelHubServiceError(err.to_string()))?;
            
                if let Some(v) = res.ip_address{
                    
                    let _ = update_cache("machine", &format!("{}", v))
                        .map_err(|err| ModelHubServiceError(err.to_string()))?;

                    v
                }else{
                    return Err(ModelHubServiceError(String::from("No IP retrurns when attempt to train model")))
                }
            },
        Some(ip)=>ip
    };
    let _ = mdl_hub_client.train_model(ip_address).await
        .map_err(|err|ModelHubServiceError(err.to_string()))?;

    Ok(())
}


pub async fn get_machine_by_machine_id(machine_id:&str) {
    // let pc = PaperSpaceClient::new();
    // let result = pc.get_machine_by_machine_id(machine_id).await
    //     .map_err(|err|serialize_success(err))?;
    // Ok(serialize_success(result))
}