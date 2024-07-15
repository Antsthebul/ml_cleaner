use ssh2::Session;
use reqwest::{self, header};
use s3::request;
use std::{env, fmt, net::{Ipv4Addr, TcpStream}};
use serde::{self, Deserialize, Serialize,
de::DeserializeOwned};

use crate::clients::file_config::Configuration;
struct PaperSpaceClientError(String);

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Machine{
    id:String,
    name:String,
    state: String,
    machine_type:String,
    public_ip_address: Option<Ipv4Addr>
}

#[derive(Deserialize, Serialize)]
struct Machines{
    machines:Vec<Machine>
}

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
        println!("cry{}", text);
        Ok( serde_json::from_str(&text).map_err(|err|PaperSpaceClientError(err.to_string()))?)

    }
    pub async fn get_machine_by_machine_id(self, machine_id:&str)->Result<Machine, PaperSpaceClientError>{
        let url = format!("{}/getMachinePublic?machineId={machine_id}",self.base_url);

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

    pub async fn get_machine_status(self, project_name:&str, machine_id:&str) -> Result<Machine, PaperSpaceClientError>{
        let url = format!("{}/getMachinePublic?machineId={machine_id}", self.base_url);

        let response = self.make_request::<Machine>(url, "get".parse().unwrap()).await?;
        println!("wtf {} ", response);
        let config = Configuration::get_configuration_file().map_err(|err|PaperSpaceClientError(err.to_string()))?;
        let mut project = Configuration::get_project_by_project_name(project_name).map_err(|err|PaperSpaceClientError(err.to_string()))?;

        if let Some(mut machine) = project.machine{
            if let Some(ip_addr) = response.public_ip_address{
                println!("attr exists!");
                machine.ip_addr = Some(ip_addr);
                project.machine = Some(machine);
            };
        };
        Configuration::update_configuration_file(config)
            .map_err(|err|PaperSpaceClientError(err.to_string()))?;
        Ok(response)
    }

    pub async fn train_model(self, ip_address:Ipv4Addr) ->  Result<serde_json::value::Value, PaperSpaceClientError>{
        let mut  session = Session::new()
            .map_err(|err|PaperSpaceClientError(err.to_string()))?;

        let tcp = TcpStream::connect(ip_address.to_string()).map_err(|err|PaperSpaceClientError(err.to_string()))?;
        
        session.set_tcp_stream(tcp);
        session.userauth_agent("bulofants").map_err(|err|PaperSpaceClientError(err.to_string()))?;

        Ok(serde_json::json!({"ok":"not done"}))
    }

}



impl fmt::Display for Machine{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}
