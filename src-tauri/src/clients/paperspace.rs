use reqwest::{self, header};
use s3::request;
use std::{env, fmt};
use serde::{self, Deserialize, Serialize,
de::DeserializeOwned};

struct PaperSpaceClientError(String);

#[derive(Deserialize)]
struct PaperSpaceServerResponse{
    status: u16,
    message:String
}

#[derive(Debug)]
enum RequestType{
    GET,
    POST,
}

struct PaperSpaceClient{
    base_url:&'static str,
    client:reqwest::Client
}

impl std::str::FromStr for RequestType{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "get"=> Ok(Self::GET),
            "post"=> Ok(Self::POST),
            _=>Err(())
        } 
    }
}
impl fmt::Display for PaperSpaceClientError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "<PaperSpaceClientError> {}", self.0)
    }
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

    async fn make_request<T: DeserializeOwned >(&self, url:String, request_type:RequestType)
        ->Result <T, PaperSpaceClientError>{
        println!("PaperSpaceClient: Sending {:?} to {:?}", request_type, url);

       let request = match request_type {
    
           RequestType::GET=>self.client.get(url),
           RequestType::POST=>self.client.post(url)
        };
        
       let response = request.send()
       .await
       .map_err(|err| PaperSpaceClientError(err.to_string()))?;
    

        match response.status(){
            // Overkill to make a struct for a single property
            reqwest::StatusCode::UNAUTHORIZED => {
                let result = response.json::<PaperSpaceServerResponse>().await.map_err(|err|PaperSpaceClientError(err.to_string()))?;
                return Err(PaperSpaceClientError(result.message))
            }
            reqwest::StatusCode::BAD_REQUEST =>{
                let result = response.json::<PaperSpaceServerResponse>().await.map_err(|err|PaperSpaceClientError(err.to_string()))?;
                return Err(PaperSpaceClientError(result.message))
            },
            _=>()
        };
        // leave as separate steps for debugging
        let text = response.text().await.unwrap();
  
        Ok( serde_json::from_str(&text).map_err(|err|PaperSpaceClientError(err.to_string()))?)

    }
    pub async fn get_machine(self, machine_id:&str)->Result<Machine, PaperSpaceClientError>{
        let mut url = self.base_url.to_owned();
        url.push_str(&format!("/getMachinePublic?machineId={}",machine_id));

        Ok(self.make_request::<Machine>(url, "get".parse().unwrap()).await?)
    }

    pub async fn get_machines(self)->Result<Vec<Machine>, PaperSpaceClientError>{
        let mut url = self.base_url.to_owned();
        url.push_str("/getMachines");

        Ok(self.make_request::<Vec<Machine>>(url, "get".parse().unwrap()).await?)
    
    }

    pub async fn handle_machine_run_state(self, machine_id:&str, action:&str)-> Result<(), PaperSpaceClientError>{
        let mut url = self.base_url.to_owned();
        url.push_str(&format!("/{}/{}", machine_id, action));

        let response = self.make_request::<serde_json::value::Value>(url, "post".parse().unwrap()).await?;
    
        print!("State chnage, {}", response);
        Ok(())
    }

}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Machine{
    id:String,
    name:String,
    state: String,
    machine_type:String,
}

#[derive(Deserialize, Serialize)]
struct Machines{
    machines:Vec<Machine>
}

#[tauri::command]
pub async fn get_status(){

}

async fn get_machine_status(machine_id:String)->Result<Machine, PaperSpaceClientError>{
    let pc = PaperSpaceClient::new();
    Ok(pc.get_machine(&machine_id).await?)

}

#[tauri::command]
pub async fn is_running(machine_id:&str)-> Result<String, String>{
    let machine = get_machine_status(machine_id.to_string()).await
        .map_err(|err| serde_json::to_string(&serde_json::json!({"error":err.to_string()})).unwrap())?;
    let res = machine.state == "ready";

    let response = serde_json::json!({"data":res});
    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn list_machines()-> Result<String, String>{
    let pc = PaperSpaceClient::new();
    let machines = pc.get_machines().await
    .map_err(|err| serde_json::to_string(&serde_json::json!({"error":err.to_string()})).unwrap())?;

    let response = serde_json::json!({"data":machines});
    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn start_machine(machine_id:&str) -> Result<String, String>{
    let pc = PaperSpaceClient::new();
    let  _ = pc.handle_machine_run_state(machine_id, "start").await
        .map_err(|err|serde_json::to_string(&serde_json::json!({"error":err.to_string()})).unwrap())?;

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