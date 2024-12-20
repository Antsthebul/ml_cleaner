use core::fmt;
use std::{net::Ipv4Addr, str::FromStr, time, env};

pub use paperspace::MachineState;

use crate::{
    database::DbClient, 
    components::get_run_environment
};

pub mod paperspace;

pub struct ClientMachineResponse {
    pub id: String,
    pub ip_address: Option<Ipv4Addr>,
    pub state: MachineState,
}

#[derive(Debug)]
pub struct ModelHubError(String);

#[derive(Debug)]
pub enum ClientType {
    PaperSpace,
}

pub trait Client {
    fn new() -> Self;
    fn train_model(
        self,
        ip_address: Ipv4Addr,
    ) -> impl std::future::Future<Output = Result<(), ModelHubError>> + Send;
    fn stop_train_model(
        &self,
        ip_address: Ipv4Addr,
    ) -> impl std::future::Future<Output = Result<(), ModelHubError>> + Send;
    fn check_training_status(
        self,
        ip_address: Ipv4Addr,
    ) -> impl std::future::Future<Output = Result<(), ModelHubError>> + Send;
    /// Returns
    fn get_training_results(
        self,
        ip_address: Ipv4Addr,
    ) -> impl std::future::Future<Output = Result<String, ModelHubError>> + Send;
    fn download_model(
        self,
        ip_address: Ipv4Addr,
        deployment_name: &str,
    ) -> impl std::future::Future<Output = Result<(), ModelHubError>> + Send;
    fn handle_machine_run_state(
        &self,
        machine_id: &str,
        action: &str,
    ) -> impl std::future::Future<Output = Result<ClientMachineResponse, ModelHubError>> + Send;
    fn get_machine_status(
        self,
        machine_id: &str,
    ) -> impl std::future::Future<Output = Result<ClientMachineResponse, ModelHubError>> + Send;
    fn get_base_url() -> String;
}

impl FromStr for ClientType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "paperspace" => Ok(Self::PaperSpace),
            _ => Err(s.to_owned()),
        }
    }
}
impl fmt::Display for ModelHubError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
pub fn create_client(client_type: ClientType) -> Result<impl Client + Clone, ModelHubError> {
    match client_type {
        ClientType::PaperSpace => Ok(paperspace::PaperSpaceClient::new()),
    }
}

fn is_machine_off(machine: ClientMachineResponse) -> bool {
    let off_states = vec![MachineState::Off];

    if off_states.contains(&machine.state) {
        return true;
    }
    false
}

pub async fn state_check_daemon(provider: String, machine_id: String, called_by:String) {
    println!("[StateCheckDaemon-{}-{}]. Started", machine_id, called_by);
    let model_hub_client = create_client(provider.parse().unwrap()).unwrap();

    loop {
        let c = model_hub_client.clone();
        let res = c.get_machine_status(&machine_id).await;

        match res {
            Err(err) => {
                println!(
                    "[StateCheckDaemon-{}]. Unable to determine state due to {}",
                    machine_id, err
                )
            }
            Ok(val) => {
                let conn = DbClient::new().await;
                if let Ok(db_client) = conn {
                    let ip_address = match &val.ip_address {
                        Some(ip) => ip.to_string(),
                        None => "".to_string(),
                    };
                    let _ = db_client
                        .execute(
                            "UPDATE machines set state=$1, ip_address=$2 where machine_id=$3",
                            &[&val.state, &ip_address, &machine_id],
                        )
                        .await
                        .unwrap();
                } else {
                    println!("Failed to connect to db")
                }

                if is_machine_off(val) {
                    println!("[Daemon-{}]. Machine is off. Exiting", machine_id);
                    return;
                }
            }
        }
        tokio::time::sleep(time::Duration::from_millis(5000)).await;
    }
}
pub fn orkestr8_run() -> String{
    let access = env::var("AWS_ACCESS_KEY").unwrap();
    let secret = env::var("AWS_SECRET_KEY").unwrap();
    let bucket = env::var("AWS_BUCKET_NAME").unwrap();

    let mut command = String::from("nohup bash -c 'pip install --upgrade orkestr8-sdk &&");

    let command_suffix = match get_run_environment(){
        crate::components::ENVIRONMENT::PRODUCTION=>format!("
            pip install --force-reinstall -v \'numpy==1.25.2\' && 
            BASE_IMAGES_DIRECTORY=~/data/images \
            BASE_RUN_LOCATION=~/data/runs \
            BASE_MODEL_PATH=~/data/model \
            orkestr8 run --aws-secret-key={secret} --aws-access-key={access} --aws-bucket-name={bucket} --model-module=main --remote-file-path=code/foodenie_ml.tar.gz --dest-file-path=foodenie_ml -y"),
        crate::components::ENVIRONMENT::LOCAL=>String::from("orkestr8 mock_run")
    };

    command.push_str(&command_suffix);
    command.push_str("' >> log.txt 2>&1 &");
    
    command
}

pub fn orkestr8_download_model(deployment_name:&str)-> String{
    let access = env::var("AWS_ACCESS_KEY").unwrap();
    let secret = env::var("AWS_SECRET_KEY").unwrap();
    let bucket = env::var("AWS_BUCKET_NAME").unwrap();

    match get_run_environment(){
        crate::components::ENVIRONMENT::LOCAL=>String::from("orkestr8 mock_run"),
        crate::components::ENVIRONMENT::PRODUCTION=>format!(
            "orkestr8 download_model S3 
            --aws-secret-key={secret} 
            --aws-access-key={access} 
            --aws-bucket-name={bucket}  
            --remote-location=data/ml_state/{deployment_name}
            --model-location=~/data/model/foodenie_resnet.pth
            "
        )
    }
}