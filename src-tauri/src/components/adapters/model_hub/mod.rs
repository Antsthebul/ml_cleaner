use core::fmt;
use std::{time, net::Ipv4Addr, str::FromStr};

use paperspace::MachineState;

use crate::database::DbClient;

mod paperspace;
pub struct ClientMachineResponse{
    pub ip_address: Option<Ipv4Addr>,
    pub state: paperspace::MachineState
}

#[derive(Debug)]
pub struct ModelHubError(String);

#[derive(Debug)]
pub enum ClientType{
    PaperSpace
}

pub trait Client {
    fn new() -> Self;
    fn train_model(self, ip_address:Ipv4Addr) -> impl std::future::Future <Output = Result<(), ModelHubError>> + Send;
    fn handle_machine_run_state(&self, machine_id:&str, action:&str) -> impl std::future::Future <Output = Result<ClientMachineResponse, ModelHubError>> + Send;
    fn get_machine_status(self, machine_id:&str)  -> impl std::future::Future <Output = Result<ClientMachineResponse, ModelHubError>> + Send;
    fn get_base_url() -> String;
}


impl FromStr for ClientType{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>{
        match s.to_lowercase().as_str() {
            "paperspace" => Ok(Self::PaperSpace),
            _=>Err(s.to_owned())
        }
    }
}
impl fmt::Display for ModelHubError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        write!(f, "{}", self.0)
    }
}
pub fn create_client(client_type:ClientType)-> Result<impl Client+Clone, ModelHubError> {
    match client_type{
        ClientType::PaperSpace=> Ok(paperspace::PaperSpaceClient::new()),
        _=>Err(ModelHubError(String::from(format!("'{:?}' is not a configured client", client_type))))
    }
}

fn is_machine_off(machine:ClientMachineResponse) -> bool{
    let off_states = vec![MachineState::Off, MachineState::Stopping];

    if off_states.contains(&machine.state){
        return true
    }
    false
}

pub async fn state_check_daemon(provider:String, machine_id:String){
    println!("[Daemon-{}]. Started", machine_id);
    let client = create_client(provider.parse().unwrap()).unwrap();

    loop{
        let c = client.clone();
        let res = c.get_machine_status(&machine_id).await;

        match res{
            Err(err) =>{println!("[Daemon-{}]. Unable to determine state due to {}", machine_id, err)},
            Ok(val)=>{
                let conn = DbClient::new().await;
                if let Ok(db_client) = conn{
                    let _ = db_client.execute("UPDATE machines set state=$1 where machine_id=$2", &[&val.state.to_string(), &machine_id]);
                }else{
                    println!("Failed to connect to db")
                }

                if is_machine_off(val){ println!("[Daemon-{}]. Machine is off. Exiting", machine_id);return}}
        }
        tokio::time::sleep(time::Duration::from_millis(5000)).await;
    }
}