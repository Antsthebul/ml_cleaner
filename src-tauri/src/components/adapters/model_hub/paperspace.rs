use reqwest::header;
use serde::{de::{DeserializeOwned, Error as DeError}, Deserialize,Deserializer, Serialize};
use serde_json::Value;
use std::{collections::HashMap, env, fmt, fs::File, io::BufReader, net::{Ipv4Addr, TcpStream}, str::FromStr, time};
use ssh2::Session;

use crate::{state_check_daemon, ClientMachineResponse};

use super::{Client, ModelHubError};

pub struct PaperSpaceClientError(String);

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
    Patch
}


impl std::str::FromStr for MachineState{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
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
            "patch"=>Ok(Self::Patch),
            _=>Err(())
        } 
    }
}
impl fmt::Display for PaperSpaceClientError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
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
    #[serde(deserialize_with="deserialize_state")]
    state: MachineState,
    machine_type:String,
    #[serde(rename="publicIp")]
    public_ip_address: Option<Ipv4Addr>
}

#[derive(Clone)]
pub struct PaperSpaceClient{
    base_url:String,
    client:reqwest::Client
}

impl Client for PaperSpaceClient{
    fn get_base_url() -> String {
        "https://api.paperspace.com/v1/machines".to_string()
    }
    fn new()->Self{
        let mut headers = header::HeaderMap::new();
   
        let api_key = env::var("PAPERSPACE_API_KEY").unwrap();
       

        headers.insert("Authorization", format!("Bearer {}", api_key).parse().unwrap());

        
        PaperSpaceClient{ 
            client:reqwest::Client::builder()
            .default_headers(headers).build().unwrap(), base_url: Self::get_base_url()
   }
   
   }

    /// Connects via SSH to invoke 'train' command
    async fn train_model(self, ip_address:Ipv4Addr) ->  Result<(), ModelHubError>{
        let mut  session = Session::new()
            .map_err(|err|ModelHubError(format!("unable to start a new session {}",err)))?;
        let addr = format!("{}:22", ip_address);
        let tcp = TcpStream::connect(addr.as_str())
            .map_err(|err|ModelHubError(format!("tcp conection failed. {}",err)))?;
        
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
        println!("[Papserspace-<handle_machine_run_state>] State change successfully invoked");
        let public_ip= data.get("publicIp");
        let state = data.get("state").unwrap();
        let mut ip_address = None;
        
        if let Some(ip_addr) = public_ip{
            if format!("{}",ip_addr) != "null".to_string(){
                ip_address = ip_addr.as_str().unwrap().parse().ok();
            }else{
                if action == "start"{
                    let mut count = 0;
                    println!("Entering poll for ip...\n");
                    while count < 10{
                        let status = self.clone().get_machine_by_machine_id(machine_id)
                        .await
                        .map_err(|err|ModelHubError(format!("Error encountered while polling for IP. {}",err)))?;
                        
                        // println!("You machine! => {}", status);
                        if status.public_ip_address.is_some(){
                            ip_address = status.public_ip_address;
                            println!("[Paperspace] Ip acquired..");
                            break
                        }
                        tokio::time::sleep(time::Duration::from_millis(20000)).await;
                        count +=1
                    }
                    if count == 5{
                        return Err(ModelHubError(String::from(format!("IP address no returned for machine {}", machine_id))));
                    }
                }
            }
        };
        if ["start", "run"].contains(&action){
            println!("[Paperspace] Invoking state poll");
            let m_id = machine_id.to_owned();
            tauri::async_runtime::spawn(async move{
                state_check_daemon("paperspace".parse().unwrap(),m_id).await;
            });
        }
      
        Ok(ClientMachineResponse{id:machine_id.to_string(),ip_address:ip_address, state:state.as_str().unwrap().parse().unwrap()})
    }    

    /// A thin wrapper around get machine by machine_id
    /// to return a 'generic' ClientMachineResponse
    async fn get_machine_status(self,  machine_id:&str) -> Result<ClientMachineResponse, ModelHubError>{
        let response = self.get_machine_by_machine_id(machine_id).await
            .map_err(|err|ModelHubError(err.to_string()))?;

        Ok(ClientMachineResponse { id:machine_id.to_string(), ip_address: response.public_ip_address, state: response.state })
    }
    
}
impl PaperSpaceClient{

    // Read token from path, other attempts to
    // signin
    fn get_token() -> String{
        let f = File::open(".cache/map.json").unwrap();
        let rdr = BufReader::new(f);
        let cache_data: serde_json::Value= serde_json::from_reader(rdr).unwrap();
        let token = cache_data.get("token").unwrap();

        token.to_string()
    }

    async fn sign_in(self){
        // On app load existence is already checked
        let api_key = env::var("PAPERSPACE_API_KEY").unwrap();
        let url = Self::get_base_url();
        let c = reqwest::Client::new();

        let res = c.get(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
        .unwrap();

    }

    async fn make_request<T: DeserializeOwned >(&self, url:String, request_type:RequestType)
        ->Result <T, PaperSpaceClientError>{
        println!("PaperSpaceClient: Sending {:?} to {:?}", request_type, url);

       let request = match request_type {
    
           RequestType::GET=>self.client.get(url),
           RequestType::POST=>self.client.post(url),
           RequestType::Patch=>self.client.patch(url)
        };
        
       let response = request.send()
       .await
       .map_err(|err| PaperSpaceClientError(err.to_string()))?;

        match response.status(){
            // Overkill to make a struct for a single property
            reqwest::StatusCode::UNAUTHORIZED => {
                let result = response.json::<Value>().await.map_err(|err|PaperSpaceClientError(err.to_string()))?;
                let h_map = serde_json::de::from_str::<HashMap<String, String>>(&result.to_string()).unwrap();
                return Err(PaperSpaceClientError(h_map.get("message").unwrap().to_owned()))
            }
            reqwest::StatusCode::BAD_REQUEST =>{
                let result = response.json::<Value>().await.map_err(|err|PaperSpaceClientError(err.to_string()))?;
                return Err(PaperSpaceClientError(result.to_string()))
            },
            _=>()
        };
        // leave as separate steps for debugging
        let text = response.text().await.unwrap();
        Ok( serde_json::from_str(&text).map_err(|err|PaperSpaceClientError(err.to_string()))?)

    }
    pub async fn get_machine_by_machine_id(self, machine_id:&str)->Result<Machine, PaperSpaceClientError>{
        let url = format!("{}/{machine_id}",self.base_url);

        Ok(self.make_request::<Machine>(url, "get".parse().unwrap()).await?)
    }

    pub async fn get_machines(self)->Result<Vec<Machine>, PaperSpaceClientError>{
        let mut url = self.base_url.to_owned();
        url.push_str("/getMachines");

        Ok(self.make_request::<Vec<Machine>>(url, "get".parse().unwrap()).await?)
    
    }





}



fn deserialize_state<'a, D>(deserializer:D)-> Result<MachineState, D::Error>
where 
    D:Deserializer<'a>
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        
        Ok(MachineState::from_str(&s)
            .map_err(|_|D::Error::custom(format!("You suck {}", s)))?)
            
    }