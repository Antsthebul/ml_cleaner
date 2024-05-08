use reqwest::{self, header};
use std::env;
use serde::{self, Deserialize, Serialize,
de::DeserializeOwned};

#[derive(Debug)]
enum RequestType{
    GET,
    POST,}

struct PaperSpaceClient{
    base_url:&'static str,
    client:reqwest::Client
}

impl PaperSpaceClient{

    pub fn new()->Self{
         let mut headers = header::HeaderMap::new();
    
        let api_key = env::var("PAPERSPACE_API_KEY").unwrap();
        headers.insert("X-Api-Key", header::HeaderValue::from_str(&api_key).unwrap());

               
        PaperSpaceClient{ 
        client:reqwest::Client::builder()
        .default_headers(headers).build().unwrap(), base_url:"https://api.paperspace.io/machines" 
    }
    
    }

    async fn make_request<T: DeserializeOwned >(&self, url:String, request_type:RequestType)->T{
        println!("Sending {:?} to {:?}", request_type, url);

       let request = match request_type {
    
           RequestType::GET=>self.client.get(url),
           RequestType::POST=>self.client.post(url)
        };
        
       
       request.send()
        .await
        .unwrap()
        .json::<T>()
        .await.unwrap()
    
    }
    pub async fn get_machine(self, machine_id:&str)-> Machine{
        let mut url = self.base_url.to_owned();
        url.push_str(&format!("/getMachinePublic?machineId={}",machine_id));

        self.make_request::<Machine>(url, RequestType::GET).await
    }

    pub async fn get_machines(self)->Vec<Machine>{
        let mut url = self.base_url.to_owned();
        url.push_str("/getMachines");

        self.make_request::<Vec<Machine>>(url, RequestType::GET).await
    
    }

    pub async fn handle_machine_run_state(self, machine_id:&str, action:&str){
        let mut url = self.base_url.to_owned();
        url.push_str(&format!("/{}/{}", machine_id, action));

        let response = self.make_request::<serde_json::value::Value>(url,RequestType::POST).await;
    
        print!("State chnage, {:?}", response)
    }

}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
struct Machine{
    id:String,
    name:String,
    state: String,
    machine_type:String
}

#[derive(Deserialize, Serialize)]
struct Machines{
    machines:Vec<Machine>
}

#[tauri::command]
pub async fn get_status(){

}

async fn get_machine_status(machine_id:String)->Machine{
    let pc = PaperSpaceClient::new();
    pc.get_machine(&machine_id).await

}

#[tauri::command]
pub async fn is_running(machine_id:&str)-> Result<String, String>{
    let machine = get_machine_status(machine_id.to_string()).await;
    let res = machine.state == "ready";

    let response = serde_json::json!({"data":res});
    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn list_machines()-> Result<String, String>{
    let pc = PaperSpaceClient::new();
    let machines = pc.get_machines().await;

    let response = serde_json::json!({"data":machines});
    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn start_machine(machine_id:&str) -> Result<String, String>{
    let pc = PaperSpaceClient::new();
    pc.handle_machine_run_state(machine_id, "start").await;

    let response = serde_json::json!({"data":"success"});

    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn stop_machine(machine_id:&str) -> Result<String, String>{
    let pc = PaperSpaceClient::new();
    pc.handle_machine_run_state(machine_id, "stop").await;

    let response = serde_json::json!({"data":"success"});

    Ok(serde_json::to_string(&response).unwrap())
}