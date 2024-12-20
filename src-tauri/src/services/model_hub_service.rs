use crate::cache_reg::update_cache;
use app::{
    components::adapters::model_hub::MachineState, create_client, database::DbClient,
    file_config::Deployment, state_check_daemon, ClientMachineResponse,
};
use serde::Serialize;
use serde_json::json;
use std::{collections::HashMap, fmt, net::Ipv4Addr};
use tauri::utils::pattern;

use super::{
    config_service, data_lake_service, image_verifier_service,
    project_service::get_project_by_project_name,
};
use rand::prelude::SliceRandom;

use app::Client;
use regex::Regex;

pub struct ModelHubServiceError(String);

#[derive(Serialize)]
pub struct TrainingData {
    epoch: u16,
    train_acc: String,
    test_acc: String,
    time: String,
    train_loss: f32,
    val_loss: f32,
    dir_name: String,
}

impl TrainingData {
    fn parse(text: String) -> Self {
        let pattern = format!(r#"(["a-z\d\._\'%]+)\s*,?"#);
        let epoch_re = Regex::new(&format!(r"epoch={}", pattern)).unwrap();
        let train_acc_re = Regex::new(&format!(r"train_acc={}", pattern)).unwrap();
        let test_acc_re = Regex::new(&format!(r"test_acc={}", pattern)).unwrap();
        let time_re = Regex::new(&format!("time={}", pattern)).unwrap();
        let train_loss_re = Regex::new(&format!(r"train_loss={}", pattern)).unwrap();
        let val_loss_re = Regex::new(&format!(r"val_loss={}", pattern)).unwrap();
        let dir_name_re = Regex::new(&format!(r"dir_name={}", pattern)).unwrap();

        let epoch = epoch_re.captures(&text).unwrap();
        let train_acc = train_acc_re.captures(&text).unwrap();
        let test_acc = test_acc_re.captures(&text).unwrap();
        let time = time_re.captures(&text).unwrap();
        let train_loss = train_loss_re.captures(&text).unwrap();
        let val_loss = val_loss_re.captures(&text).unwrap();
        let dir_name = dir_name_re.captures(&text).unwrap();

        TrainingData {
            epoch: epoch.get(1).unwrap().as_str().to_owned().parse().unwrap(),
            train_acc: train_acc.get(1).unwrap().as_str().to_owned(),
            test_acc: test_acc.get(1).unwrap().as_str().to_owned(),
            time: time.get(1).unwrap().as_str().to_owned(),
            train_loss: train_loss
                .get(1)
                .unwrap()
                .as_str()
                .to_owned()
                .parse()
                .unwrap(),
            val_loss: val_loss
                .get(1)
                .unwrap()
                .as_str()
                .to_owned()
                .parse()
                .unwrap(),
            dir_name: dir_name.get(1).unwrap().as_str().to_owned(),
        }
    }
}

impl fmt::Display for ModelHubServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
/// Generates test/train data for related project.
/// For now we generate from scratch
pub async fn generate_test_train_data(
    project_name: &str,
    train_pct: Option<f64>,
) -> Result<(), ModelHubServiceError> {
    let _ = get_project_by_project_name(project_name)
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let image_map = image_verifier_service::list_all_images_by_class()
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let all_images = shuffle_images(&image_map);

    let mut pct = 0.8;

    if let Some(val) = train_pct {
        pct = val
    };

    let split_data = split_dataset(&all_images, pct);

    write_dataset_to_repo(project_name, split_data)
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    println!("training {} images", all_images.len());

    Ok(())
}

fn shuffle_images(images: &HashMap<String, Vec<String>>) -> Vec<(String, String)> {
    let mut all_images: Vec<(String, String)> = Vec::new();
    for set_ in images.into_iter() {
        let c_name = set_.0;
        for im in set_.1 {
            all_images.push((c_name.to_owned(), im.to_owned()));
        }
    }

    let mut rng = rand::thread_rng();
    all_images.shuffle(&mut rng);
    all_images
}

/// Splits the dataset base on the training data percentage amount (`train_pct`) provided
fn split_dataset(
    data: &Vec<(String, String)>,
    train_pct: f64,
) -> HashMap<String, &[(String, String)]> {
    let len_of_data = data.len() as f64;
    let limit = (len_of_data * train_pct) as usize;

    let train = &data[0..limit];
    let test = &data[limit..data.len()];

    let mut hm = HashMap::new();
    hm.insert("train".to_owned(), train);
    hm.insert("test".to_owned(), test);
    hm
}
/// Writes the test/train data, provided as a HashMap, to the data lake repository
async fn write_dataset_to_repo(
    project_name: &str,
    data: HashMap<String, &[(String, String)]>,
) -> Result<(), ModelHubServiceError> {
    for data_type in vec!["train", "test"] {
        let data = data.get(data_type).unwrap();

        let file_path = format!("data/base/{}.json", data_type);

        let train_contents = serde_json::to_vec(&json!({data_type:data})).unwrap();

        data_lake_service::write_file(project_name, &file_path, train_contents.as_slice())
            .await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;

        println!("Successfully wrote to {}", file_path);
    }

    Ok(())
}
/// Retreives a list of status's of all machines by looking up the related model hub
/// vendor, and accessing their API.
pub async fn get_machine_status(
    deployment_name: &str,
    project_name: &str,
) -> Result<Vec<ClientMachineResponse>, ModelHubServiceError> {
    let via_api = false;

    let proj = config_service::get_project_by_project_name(project_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let dep = proj
        .get_project_deployment(deployment_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let tmp = dep.machines.first().unwrap();

    let client = create_client(tmp.provider.parse().unwrap()).unwrap();
    let db_client = DbClient::new().await.unwrap();
    let mut results = Vec::new();

    for m in &dep.machines {
        let mut status: Option<_> = None;

        if via_api {
            status = Some(
                client
                    .clone()
                    .get_machine_status(&m.id)
                    .await
                    .map_err(|err| ModelHubServiceError(err.to_string()))?,
            );
        } else {
            let rows = db_client
                .query(
                    "SELECT ip_address, state FROM machines where machine_id=$1 ",
                    &[&m.id],
                )
                .await
                .map_err(|err| ModelHubServiceError(err.to_string()))?;

            let row = rows.first().unwrap();
            let ip_addr_str = row.get::<usize, Option<&str>>(0);
            let mut ip_address = None;
            if let Some(ip_addr) = ip_addr_str{

                if let Ok(ip) = ip_addr.parse::<Ipv4Addr>() {
                    ip_address = Some(ip)
                };
            };
            status = Some(ClientMachineResponse {
                id: m.id.to_owned(),
                ip_address: ip_address,
                state: row.get(1),
            });
        }
        if let Some(s) = status {
            results.push(s)
        } else {
            println!("No satus found!")
        }
    }
    Ok(results)
}

// TODO: We might not want 'list' machines, in the way that the UI
// allows. List machines works by API key
// (ie. "as a user these are your amchines") whereas Orkestr8ML works by
// project.
pub async fn list_machines() {
    // let pc = PaperSpaceClient::new();
    // let machines = pc.get_machines().await
    // .map_err(|err|serialize_error(err.to_string()))?;

    // let response = serde_json::json!({"data":machines});
    // Ok(serde_json::to_string(&response).unwrap())
}

pub async fn start_or_stop_machine(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
    machine_action: &str,
) -> Result<(), ModelHubServiceError> {
    let proj = config_service::get_project_by_project_name(project_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let dep = proj
        .get_project_deployment(deployment_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let mach = dep
        .get_machine_by_machine_id(machine_id)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();

    if machine_action == "start" {
        let _ = mdl_hub_client
            .handle_machine_run_state(machine_id, "start")
            .await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;

        let db_client = DbClient::new()
            .await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;

        // Should use correct MachineState ENUM here
        let _ = db_client.execute("INSERT into machines (machine_id, state) VALUES ($1, $2) ON CONFLICT(machine_id) DO UPDATE set state=$2", &[&machine_id, &MachineState::Off]).await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;

        let m_id = machine_id.to_owned();
        tauri::async_runtime::spawn(async move {
            println!("[Paperspace] Invoking state poll");

            state_check_daemon(mach.provider.parse().unwrap(), m_id, String::from("machine_start")).await;
        });
    } else if machine_action == "stop" {
        let _ = mdl_hub_client
            .handle_machine_run_state(machine_id, "stop")
            .await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;
    }
    Ok(())
}

/// Download a machine to a given destination
pub async fn download_model(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<(), ModelHubServiceError> {
    let dep = get_deployment(deployment_name, project_name)?;

    let mach = dep
        .get_machine_by_machine_id(machine_id)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();

    let _ = mdl_hub_client.download_model(mach.ip_addr.unwrap(), deployment_name);

    Ok(())
}
pub async fn get_training_results(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<TrainingData, ModelHubServiceError> {
    let dep = get_deployment(deployment_name, project_name)?;

    let mach = dep
        .get_machine_by_machine_id(machine_id)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();
    let db_client = DbClient::new()
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let rows = db_client
        .query(
            "SELECT ip_address FROM machines where machine_id=$1",
            &[&machine_id],
        )
        .await
        .unwrap();
    let ip_address = rows[0].get::<usize, String>(0);

    let results = mdl_hub_client
        .get_training_results(ip_address.parse().unwrap())
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    println!("Traing result => {}", results);

    if results.contains("connection refused") {
        return Err(ModelHubServiceError("connection refused".to_string()));
    };
    if results.trim().is_empty() {
        return Err(ModelHubServiceError("No data".to_owned()));
    };
    Ok(TrainingData::parse(results))
}

/// This WILL NOT shutdown the machine. Simply invokes a 'shutdown' command to
/// training process
pub async fn stop_train_model(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<(), ModelHubServiceError> {
    // We use the 'minimal' retreival method, since we only need provider

    let dep = get_deployment(deployment_name, project_name)?;
    let mach = dep
        .get_machine_by_machine_id(machine_id)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();

    let db_client = DbClient::new()
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let rows = db_client
        .query(
            "SELECT ip_address FROM machines where machine_id=$1",
            &[&machine_id],
        )
        .await
        .unwrap();
    let ip_address = rows[0].get::<usize, String>(0);
    println!("[ModelHub]. Invoked Stopped Training on IP {}", ip_address);

    let _ = update_cache("machine", &format!("{}", ip_address))
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let _ = mdl_hub_client
        .stop_train_model(ip_address.parse().unwrap())
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let _ = db_client
        .execute(
            "UPDATE machines set state=$1 where machine_id=$2",
            &[&MachineState::Ready, &machine_id],
        )
        .await
        .unwrap();
    Ok(())
}
pub async fn train_model(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<(), ModelHubServiceError> {
    // We use the 'minimal' retreival method, since we only need provider

    let dep = get_deployment(deployment_name, project_name)?;

    let mach = dep
        .get_machine_by_machine_id(machine_id)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();

    let db_client = DbClient::new()
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let rows = db_client
        .query(
            "SELECT ip_address FROM machines where machine_id=$1",
            &[&machine_id],
        )
        .await
        .unwrap();
    let ip_address = rows[0].get::<usize, String>(0);
    println!("[ModelHub]. Training on IP {}", ip_address);

    let _ = update_cache("machine", &format!("{}", ip_address))
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    // SSH into server, and run train command
    let _ = mdl_hub_client
        .train_model(ip_address.parse().unwrap())
        .await
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    let _ = db_client
        .execute(
            "UPDATE machines set state=$1 where machine_id=$2",
            &[&MachineState::Training, &machine_id],
        )
        .await
        .unwrap();
    Ok(())
}

/// Re-usable utility function to grabs a project by name and returns
/// a deployment matching the given name
fn get_deployment(
    deployment_name: &str,
    project_name: &str,
) -> Result<Deployment, ModelHubServiceError> {
    let proj = config_service::get_project_by_project_name(project_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?;

    Ok(proj
        .get_project_deployment(deployment_name)
        .map_err(|err| ModelHubServiceError(err.to_string()))?)
}
