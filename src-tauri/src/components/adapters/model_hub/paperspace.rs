use reqwest::header;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::{env, fmt, net::{Ipv4Addr, TcpStream}};
use ssh2::Session;

use crate::ClientMachineResponse;

use super::{Client, ModelHubError};

struct PaperSpaceClientError(String);

#[derive(Serialize, Deserialize, Debug, Clone,PartialEq)]
pub enum MachineState{
    Off, 
    Starting,
    Stopping,
    Restarting,
    ServiceReady,
    Ready,
    Upgrading,
    Provisioning

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


impl std::str::FromStr for MachineState{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "off" => Ok(Self::Off),
            "starting"=> Ok(Self::Starting), 
            "stopping"=> Ok(Self::Stopping),
            "restarting" => Ok(Self::Restarting),
            "serviceready" => Ok(Self::ServiceReady),
            "ready" => Ok(Self::Ready),
            "upgrading"=>Ok(Self::Upgrading),
            "provisioning" => Ok(Self::Provisioning),
            _ => Err(())
        }
    }
}
impl fmt::Display for MachineState{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self{
            Self::Ready=>write!(f, "{}","ready"),
            Self::Off=>write!(f, "{}", "off"),
            Self::Provisioning=>write!(f, "{}","provisioning"),
            Self::Restarting=>write!(f, "{}","restarting"),
            Self::ServiceReady=>write!(f, "{}","ServiceReady"),
            Self::Stopping=>write!(f, "{}","stopping"),
            Self::Upgrading=>write!(f, "{}","upgrading"),
            Self::Starting=>write!(f, "{}", "starting")
        }
    }
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




impl fmt::Display for Machine{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"{:?}", self)
    }
}


#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Machine{
    id:String,
    name:String,
    state: MachineState,
    machine_type:String,
    public_ip_address: Option<Ipv4Addr>
}

#[derive(Clone)]
pub struct PaperSpaceClient{
    base_url:&'static str,
    client:reqwest::Client
}

impl Client for PaperSpaceClient{
    fn new()->Self{
        let mut headers = header::HeaderMap::new();
   
       let api_key = env::var("PAPERSPACE_API_KEY").unwrap();
       headers.insert("X-Api-Key", header::HeaderValue::from_str(&api_key).unwrap());

              
       PaperSpaceClient{ 
       client:reqwest::Client::builder()
       .default_headers(headers).build().unwrap(), base_url:"https://api.paperspace.io/machines" 
   }
   
   }

    /// Connects via SSH to invoke 'train' command
    async fn train_model(self, ip_address:Ipv4Addr) ->  Result<(), ModelHubError>{
        let mut  session = Session::new()
            .map_err(|err|ModelHubError(err.to_string()))?;

        let tcp = TcpStream::connect(ip_address.to_string()).map_err(|err|ModelHubError(err.to_string()))?;
        
        session.set_tcp_stream(tcp);
        session.userauth_agent("bulofants").map_err(|err|ModelHubError(err.to_string()))?;

        Ok(())
    }

    /// Recieves an action to be invoked for the machine. Returns
    /// Machine info on success
    async fn handle_machine_run_state(&self, machine_id:&str, action:&str)-> Result<ClientMachineResponse, ModelHubError>{
        println!("Sending request to change state for machine {} to {}", machine_id, action);
        let mut url = self.base_url.to_owned();
        url.push_str(&format!("/{}/{}", machine_id, action));

        let response = self.make_request::<serde_json::value::Value>(url, "patch".parse().unwrap()).await
            .map_err(|err|ModelHubError(err.to_string()))?;
        
        let data = response.get("data").unwrap();
        print!("\nState chnage completed, {}", data);
        let public_ip= data.get("publicIp");
        let state = data.get("state").unwrap();
        let mut ip_address = None;
        
        if let Some(ip_addr) = public_ip{
            ip_address = Some(ip_addr.to_string().parse().unwrap());
        };
        
        Ok(ClientMachineResponse{ip_address:ip_address, state:state.as_str().unwrap().parse().unwrap()})
    }    

    /// Returns the machine status for given machine_id
    async fn get_machine_status(self,  machine_id:&str) -> Result<ClientMachineResponse, ModelHubError>{
        let url = format!("{}/getMachinePublic?machineId={machine_id}", self.base_url);

        let response = self.make_request::<Machine>(url, "get".parse().unwrap()).await
            .map_err(|err|ModelHubError(err.to_string()))?;

        Ok(ClientMachineResponse { ip_address: response.public_ip_address, state: response.state })
    }
    
}
impl PaperSpaceClient{



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





}



