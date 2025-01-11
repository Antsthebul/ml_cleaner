use core::fmt;
use std::{env, net::Ipv4Addr, str::FromStr, time};

use deadpool_postgres::Pool;
pub use paperspace::MachineState;

use crate::client_adapters::{
    database::{
        machine_db::{Machine, MachineDb},
        AsyncDbClient, PGClient,
    },
    get_run_environment,
};

pub mod paperspace;

pub struct ClientMachineResponse {
    pub id: String,
    pub ip_address: Option<Ipv4Addr>,
    pub state: MachineState,
}

#[derive(Debug)]
pub struct ModelHubError(String);

impl Into<String> for ModelHubError {
    fn into(self) -> String {
        self.0
    }
}

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

fn is_machine_off(machine_state: MachineState) -> bool {
    let off_states = vec![MachineState::Off];

    if off_states.contains(&machine_state) {
        return true;
    }
    false
}

pub async fn state_check_daemon(pool: Pool, machine: Machine, called_by: String) {
    println!(
        "[StateCheckDaemon-{}-{}]. Started",
        machine.machine_id, called_by
    );
    let model_hub_client = create_client(machine.provider.parse().unwrap()).unwrap();

    loop {
        let c = model_hub_client.clone();
        let res = c
            .get_machine_status(machine.machine_id.to_string().as_str())
            .await;

        match res {
            Err(err) => {
                println!(
                    "[StateCheckDaemon-{}]. Unable to determine state due to {}",
                    machine.machine_id, err
                )
            }
            Ok(machine_response) => {
                let machine_db = MachineDb {
                    client: pool.get().await.unwrap(),
                };

                let ip_address = match &machine_response.ip_address {
                    Some(ip) => Some(ip.to_string().parse().unwrap()),
                    None => None,
                };


                let new_machine = Machine {
                    provider: machine.provider.to_owned(),
                    model: machine.model.to_owned(),
                    price: machine.price,
                    machine_id: machine.machine_id.to_owned(),
                    state: machine_response.state.clone(),
                    ip_address: ip_address,
                    project_id: machine.project_id,
                    deployment_id: machine.deployment_id
                };
                let _ = machine_db.update_machine(new_machine).await.unwrap();

                if is_machine_off(machine_response.state) {
                    println!("[Daemon-{}]. Machine is off. Exiting", machine.machine_id);
                    return;
                }
            }
        }
        tokio::time::sleep(time::Duration::from_millis(5000)).await;
    }
}
pub fn orkestr8_run() -> String {
    let access = env::var("AWS_ACCESS_KEY").unwrap();
    let secret = env::var("AWS_SECRET_KEY").unwrap();
    let bucket = env::var("AWS_BUCKET_NAME").unwrap();
    let log_file = get_log_file();

    let mut command = format!(
        r#"nohup bash -c 'echo "Downloading Orkestr8" >> {log_file} && pip install --upgrade orkestr8-sdk >> {log_file} 2>&1 &&"#
    );

    let command_suffix = match get_run_environment() {
        crate::client_adapters::ENVIRONMENT::PRODUCTION => format!(
            r#"echo "Invoke Orkestr8 run" >> {log_file} &&
            pip install --force-reinstall -v \"numpy==1.25.2\" && 
            BASE_IMAGES_DIRECTORY=~/data/images \
            BASE_RUN_LOCATION=~/data/runs \
            BASE_MODEL_PATH=~/data/model \
            orkestr8 run --aws-secret-key={secret} --aws-access-key={access} --aws-bucket-name={bucket} --model-module=main --remote-file-path=code/foodenie_ml.tar.gz --dest-file-path=foodenie_ml -y"#
        ),
        crate::client_adapters::ENVIRONMENT::LOCAL => String::from(
            r#"echo "Invoke Orkestr8 mock_run" >> .ml_cleaner.log && orkestr8 mock_run"#,
        ),
    };

    command.push_str(&command_suffix);
    command.push_str("'  &");
    println!("invoked orkstr8 run command as: {}", command);
    command
}

pub fn orkestr8_download_model(deployment_name: &str) -> String {
    let access = env::var("AWS_ACCESS_KEY").unwrap();
    let secret = env::var("AWS_SECRET_KEY").unwrap();
    let bucket = env::var("AWS_BUCKET_NAME").unwrap();

    match get_run_environment() {
        crate::client_adapters::ENVIRONMENT::LOCAL => format!(
            "
            echo \'Invoking Orkestr8 mock_run...\' >> {} && orkestr8 mock_run",
            get_log_file()
        ),
        crate::client_adapters::ENVIRONMENT::PRODUCTION => format!(
            "orkestr8 download_model S3 
            --aws-secret-key={secret} 
            --aws-access-key={access} 
            --aws-bucket-name={bucket}  
            --remote-location=data/ml_state/{deployment_name}
            --model-location=~/data/model/foodenie_resnet.pth
            "
        ),
    }
}

fn get_log_file() -> String {
    ".ml_cleaner.log".to_string()
}
