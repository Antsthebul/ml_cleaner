use core::fmt;
use std::{net::Ipv4Addr, str::FromStr};

mod paperspace;
pub struct ClientMachineResponse{
    pub ip_address: Option<Ipv4Addr>,
    pub state: String
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
}

impl FromStr for ClientType{
    type Err = (String);

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
