use reqwest::header;
use serde::{
    de::{DeserializeOwned, Error as DeError},
    Deserialize, Deserializer, Serialize,
};
use serde_json::Value;
use ssh::{create_session, LocalSession, SessionBroker, SessionConnector};
use std::{
    collections::HashMap,
    env, fmt,
    fs::{self, File},
    io::BufReader,
    net::{Ipv4Addr, TcpStream},
    process::Command,
    str::FromStr,
    thread, time,
};

use super::{Client, ModelHubError};
use crate::{state_check_daemon, ClientMachineResponse};
use postgres_types::{FromSql, ToSql};

pub struct PaperSpaceClientError(String);

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, ToSql, FromSql)]
#[postgres(name = "machine_state")]
pub enum MachineState {
    #[postgres(name = "off")]
    Off,
    #[postgres(name = "starting")]
    Starting,
    #[postgres(name = "stopping")]
    Stopping,
    #[postgres(name = "restarting")]
    Restarting,
    #[postgres(name = "serviceready")]
    ServiceReady,
    #[postgres(name = "ready")]
    Ready,
    #[postgres(name = "upgrading")]
    Upgrading,
    #[postgres(name = "provisioning")]
    Provisioning,
    #[postgres(name = "training")]
    Training,
}

#[derive(Deserialize, Serialize)]
struct Machines {
    machines: Vec<Machine>,
}

#[derive(Deserialize)]
struct PaperSpaceServerResponse {
    status: u16,
    message: String,
}

#[derive(Debug)]
enum RequestType {
    GET,
    POST,
    Patch,
}

impl std::str::FromStr for MachineState {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "off" => Ok(Self::Off),
            "starting" => Ok(Self::Starting),
            "stopping" => Ok(Self::Stopping),
            "restarting" => Ok(Self::Restarting),
            "serviceready" => Ok(Self::ServiceReady),
            "ready" => Ok(Self::Ready),
            "upgrading" => Ok(Self::Upgrading),
            "provisioning" => Ok(Self::Provisioning),
            _ => Err(()),
        }
    }
}

impl fmt::Display for MachineState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ready => write!(f, "{}", "ready"),
            Self::Off => write!(f, "{}", "off"),
            Self::Provisioning => write!(f, "{}", "provisioning"),
            Self::Restarting => write!(f, "{}", "restarting"),
            Self::ServiceReady => write!(f, "{}", "ServiceReady"),
            Self::Stopping => write!(f, "{}", "stopping"),
            Self::Upgrading => write!(f, "{}", "upgrading"),
            Self::Starting => write!(f, "{}", "starting"),
            Self::Training => write!(f, "{}", "training"),
        }
    }
}

impl std::str::FromStr for RequestType {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "get" => Ok(Self::GET),
            "post" => Ok(Self::POST),
            "patch" => Ok(Self::Patch),
            _ => Err(()),
        }
    }
}

impl fmt::Display for PaperSpaceClientError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Display for Machine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Machine {
    id: String,
    name: String,
    #[serde(deserialize_with = "deserialize_state")]
    state: MachineState,
    machine_type: String,
    #[serde(rename = "publicIp")]
    public_ip_address: Option<Ipv4Addr>,
}

#[derive(Clone)]
pub struct PaperSpaceClient {
    base_url: String,
    client: reqwest::Client,
}
fn create_ssh_session_local(
    ip_address: Ipv4Addr,
) -> Result<LocalSession<TcpStream>, PaperSpaceClientError> {
    Ok(create_ssh_session(ip_address)?.run_local())
}
fn create_ssh_session_backend(
    ip_address: Ipv4Addr,
) -> Result<SessionBroker, PaperSpaceClientError> {
    Ok(create_ssh_session(ip_address)?.run_backend())
}

fn create_ssh_session(
    ip_address: Ipv4Addr,
) -> Result<SessionConnector<TcpStream>, PaperSpaceClientError> {
    create_session()
        .username("paperspace")
        .private_key_path("C:/Users/Antho/.ssh/id_rsa")
        .connect(format!("{}:22", ip_address))
        .map_err(|err| PaperSpaceClientError(err.to_string()))
}

impl Client for PaperSpaceClient {
    fn get_base_url() -> String {
        "https://api.paperspace.com/v1/machines".to_string()
    }
    fn new() -> Self {
        let mut headers = header::HeaderMap::new();

        let api_key = env::var("PAPERSPACE_API_KEY").unwrap();

        headers.insert(
            "Authorization",
            format!("Bearer {}", api_key).parse().unwrap(),
        );

        PaperSpaceClient {
            client: reqwest::Client::builder()
                .default_headers(headers)
                .build()
                .unwrap(),
            base_url: Self::get_base_url(),
        }
    }
    async fn stop_train_model(&self, ip_address: Ipv4Addr) -> Result<(), ModelHubError> {
        let mut s =
            create_ssh_session_local(ip_address).map_err(|err| ModelHubError(err.to_string()))?;

        let exec = s.open_exec().unwrap();

        let command = format!("orkestr8 stop");
        let res: Vec<u8> = exec.send_command(&command).unwrap();

        println!("Result of stop: {:?}", String::from_utf8(res).unwrap());
        // Close session.
        s.close();
        println!("Training stopped!");
        Ok(())
    }
    /// Connects via SSH to invoke 'train' command
    async fn train_model(self, ip_address: Ipv4Addr) -> Result<(), ModelHubError> {
        let mut s =
            create_ssh_session_backend(ip_address).map_err(|err| ModelHubError(err.to_string()))?;
        let access = env::var("AWS_ACCESS_KEY").unwrap();
        let secret = env::var("AWS_SECRET_KEY").unwrap();
        let bucket = env::var("AWS_BUCKET_NAME").unwrap();

        let mut exec = s.open_exec().unwrap();
        thread::spawn(move || {
            let command = format!("nohup bash -c '{{ pip install --upgrade orkestr8-sdk &&
                pip install --force-reinstall -v \'numpy==1.25.2\' && 
                BASE_IMAGES_DIRECTORY=~/data/images \
                BASE_RUN_LOCATION=~/data/runs \
                BASE_MODEL_PATH=~/data/model \
                orkestr8 run --aws-secret-key={secret} --aws-access-key={access} --aws-bucket-name={bucket} --model-module=main --remote-file-path=code/foodenie_ml.tar.gz --dest-file-path=foodenie_ml -y; }}' >> log.txt 2>&1 &");
            let _ = exec.send_command(&command);
        });

        // Close session.
        s.close();
        println!("Training started!");
        Ok(())
    }
    async fn check_training_status(self, ip_address: Ipv4Addr) -> Result<(), ModelHubError> {
        let mut s =
            create_ssh_session_backend(ip_address).map_err(|err| ModelHubError(err.to_string()))?;

        let mut exec = s.open_exec().unwrap();

        let command = format!("orkestr8 check");
        let result = exec.send_command(&command);
        println!("Got result {:?}", result);
        s.close();

        Ok(())
    }

    async fn get_training_results(self, ip_address: Ipv4Addr) -> Result<String, ModelHubError> {
        let mut s =
            create_ssh_session_local(ip_address).map_err(|err| ModelHubError(err.to_string()))?;
        let exec = s.open_exec().unwrap();
        let res = exec.send_command("orkestr8 poll").unwrap();

        let data = String::from_utf8(res).unwrap();
        s.close();
        Ok(data)
    }

    async fn download_model(
        self,
        ip_address: Ipv4Addr,
        deployment_name: &str,
    ) -> Result<(), ModelHubError> {
        let mut s =
            create_ssh_session_backend(ip_address).map_err(|err| ModelHubError(err.to_string()))?;

        let access = env::var("AWS_ACCESS_KEY").unwrap();
        let secret = env::var("AWS_SECRET_KEY").unwrap();
        let bucket = env::var("AWS_BUCKET_NAME").unwrap();

        let mut exec = s.open_exec().unwrap();

        let command = format!(
            "orkestr8 download_model S3 
            --aws-secret-key={secret} 
            --aws-access-key={access} 
            --aws-bucket-name={bucket}  
            --remote-location=data/ml_state/{deployment_name}
            --model-location=~/data/model/foodenie_resnet.pth
            "
        );
        let _ = exec.send_command(&command);

        Ok(())
    }
    /// Recieves an action to be invoked for the machine. Returns
    /// Machine info on success
    async fn handle_machine_run_state(
        &self,
        machine_id: &str,
        action: &str,
    ) -> Result<ClientMachineResponse, ModelHubError> {
        println!(
            "Sending request to change state for machine {} to {}",
            machine_id, action
        );
        let mut url = self.base_url.to_owned();
        url.push_str(&format!("/{}/{}", machine_id, action));

        // Send a cancel request to server
        // to kill training process
        if action == "stop" {
            let machine = self
                .get_machine_by_machine_id(machine_id)
                .await
                .map_err(|err| ModelHubError(err.to_string()))?;

            let _ = self
                .stop_train_model(machine.public_ip_address.unwrap())
                .await
                .unwrap();
        }

        let response = self
            .make_request::<serde_json::value::Value>(url, "patch".parse().unwrap())
            .await
            .map_err(|err| ModelHubError(err.to_string()))?;

        let data = response.get("data").unwrap();
        println!("[Papserspace-<handle_machine_run_state>] State change successfully invoked");
        let public_ip = data.get("publicIp");
        let state = data.get("state").unwrap();
        let mut ip_address = None;

        if let Some(ip_addr) = public_ip {
            if format!("{}", ip_addr) != "null".to_string() {
                ip_address = ip_addr.as_str().unwrap().parse().ok();
            } else {
                if action == "start" {
                    let mut count = 0;
                    println!("Entering poll for ip...\n");
                    while count < 10 {
                        let status = self
                            .clone()
                            .get_machine_by_machine_id(machine_id)
                            .await
                            .map_err(|err| {
                                ModelHubError(format!(
                                    "Error encountered while polling for IP. {}",
                                    err
                                ))
                            })?;

                        // println!("You machine! => {}", status);
                        if status.public_ip_address.is_some() {
                            ip_address = status.public_ip_address;
                            println!("[Paperspace] Ip acquired..");
                            break;
                        }
                        tokio::time::sleep(time::Duration::from_millis(20000)).await;
                        count += 1
                    }
                    if count == 5 {
                        return Err(ModelHubError(String::from(format!(
                            "IP address no returned for machine {}",
                            machine_id
                        ))));
                    }
                }
            }
        };

        Ok(ClientMachineResponse {
            id: machine_id.to_string(),
            ip_address: ip_address,
            state: state.as_str().unwrap().parse().unwrap(),
        })
    }

    /// A thin wrapper around get machine by machine_id
    /// to return a 'generic' ClientMachineResponse
    async fn get_machine_status(
        self,
        machine_id: &str,
    ) -> Result<ClientMachineResponse, ModelHubError> {
        let response = self
            .get_machine_by_machine_id(machine_id)
            .await
            .map_err(|err| ModelHubError(err.to_string()))?;

        let mut state = response.state;
        if state == MachineState::Ready {
            if let Ok(mut s) = create_ssh_session_local(response.public_ip_address.unwrap())
                .map_err(|err| ModelHubError(err.to_string()))
            {
                let exec = s.open_exec().unwrap();
                let res: Vec<u8> = exec.send_command("orkestr8 check").unwrap();
                let output = String::from_utf8(res).unwrap();
                println!("Ya outputtter {}-> {}", output, state);
                match output.trim() {
                    "ACTIVE" => state = MachineState::Training,
                    "INACTIVE" => state = MachineState::Ready,
                    x => {
                        println!("Got unknown output {:?}. Default to READY", x);
                        state = MachineState::Ready
                    }
                }

                s.close()
            }
        }

        Ok(ClientMachineResponse {
            id: machine_id.to_string(),
            ip_address: response.public_ip_address,
            state: state,
        })
    }
}
impl PaperSpaceClient {
    async fn make_request<T: DeserializeOwned>(
        &self,
        url: String,
        request_type: RequestType,
    ) -> Result<T, PaperSpaceClientError> {
        println!("PaperSpaceClient: Sending {:?} to {:?}", request_type, url);

        let request = match request_type {
            RequestType::GET => self.client.get(url),
            RequestType::POST => self.client.post(url),
            RequestType::Patch => self.client.patch(url),
        };

        let response = request
            .send()
            .await
            .map_err(|err| PaperSpaceClientError(err.to_string()))?;

        match response.status() {
            // Overkill to make a struct for a single property
            reqwest::StatusCode::UNAUTHORIZED => {
                let result = response
                    .json::<Value>()
                    .await
                    .map_err(|err| PaperSpaceClientError(err.to_string()))?;
                let h_map =
                    serde_json::de::from_str::<HashMap<String, String>>(&result.to_string())
                        .unwrap();
                return Err(PaperSpaceClientError(
                    h_map.get("message").unwrap().to_owned(),
                ));
            }
            reqwest::StatusCode::BAD_REQUEST => {
                let result = response
                    .json::<Value>()
                    .await
                    .map_err(|err| PaperSpaceClientError(err.to_string()))?;
                return Err(PaperSpaceClientError(result.to_string()));
            }
            _ => (),
        };
        // leave as separate steps for debugging
        let text = response.text().await.unwrap();
        Ok(serde_json::from_str(&text).map_err(|err| PaperSpaceClientError(err.to_string()))?)
    }
    pub async fn get_machine_by_machine_id(
        &self,
        machine_id: &str,
    ) -> Result<Machine, PaperSpaceClientError> {
        let url = format!("{}/{machine_id}", self.base_url);

        Ok(self
            .make_request::<Machine>(url, "get".parse().unwrap())
            .await?)
    }

    pub async fn get_machines(self) -> Result<Vec<Machine>, PaperSpaceClientError> {
        let mut url = self.base_url.to_owned();
        url.push_str("/getMachines");

        Ok(self
            .make_request::<Vec<Machine>>(url, "get".parse().unwrap())
            .await?)
    }
}

fn deserialize_state<'a, D>(deserializer: D) -> Result<MachineState, D::Error>
where
    D: Deserializer<'a>,
{
    let s: String = Deserialize::deserialize(deserializer)?;

    Ok(MachineState::from_str(&s).map_err(|_| D::Error::custom(format!("You suck {}", s)))?)
}
