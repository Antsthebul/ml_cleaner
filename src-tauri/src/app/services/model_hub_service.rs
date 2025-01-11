use crate::cache_reg::update_cache;
use deadpool_postgres::Pool;
use ml_cleaner::client_adapters::{
    database::{
        activity_log_db::{self, ActivityLogDb},
        machine_db::{Machine, MachineDb, Queryable},
        AsyncDbClient, PGClient,
    },
    model_hub::{create_client, state_check_daemon, Client, ClientMachineResponse, MachineState},
    models::Deployment,
    time_series_repo::{insert_record, TrainingData},
};

use serde_json::json;
use std::{collections::HashMap, fmt, net::Ipv4Addr};

use super::{
    config_service, data_lake_service, image_verifier_service,
    project_service::get_project_by_project_name,
};
use rand::prelude::SliceRandom;

pub struct ModelHubServiceError(String);

impl fmt::Display for ModelHubServiceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Into<String> for ModelHubServiceError {
    fn into(self) -> String {
        self.0
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
    let db_client = PGClient::new().await.unwrap();
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
            // Query local dev server
            let rows = db_client
                .query(
                    "SELECT ip_address, state FROM machines where machine_id=$1 ",
                    &[&m.id],
                )
                .await
                .map_err(|err| ModelHubServiceError(err.to_string()))?;

            if let Some(row) = rows.first() {
                let ip_addr_str = row.get::<usize, Option<&str>>(0);
                let mut ip_address = None;
                if let Some(ip_addr) = ip_addr_str {
                    if let Ok(ip) = ip_addr.parse::<Ipv4Addr>() {
                        ip_address = Some(ip)
                    };
                };
                status = Some(ClientMachineResponse {
                    id: m.id.to_owned(),
                    ip_address: ip_address,
                    state: row.get(1),
                });
            } else {
                println!("Machine ID not found {:?}", &m.id);
            };
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
// pub async fn list_machines() {
// let pc = PaperSpaceClient::new();
// let machines = pc.get_machines().await
// .map_err(|err|serialize_error(err.to_string()))?;

// let response = serde_json::json!({"data":machines});
// Ok(serde_json::to_string(&response).unwrap())
// }

pub async fn start_or_stop_machine(
    pool: Pool,
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
    machine_action: &str,
) -> Result<(), ModelHubServiceError> {
    let mach_repo = MachineDb {
        client: pool.get().await.unwrap(),
    };
    let mach = mach_repo
        .get_machine_by(vec![(Queryable::Id ,&machine_id)])
        .await
        .map_err(|err| ModelHubServiceError(format!("failed to retreive machine. {err}")))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();

    if machine_action == "start" {
        let _ = mdl_hub_client
            .handle_machine_run_state(machine_id, "start")
            .await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;

        let db_client = PGClient::new()
            .await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;

        // Should use correct MachineState ENUM here
        let _ = db_client.execute("INSERT into machines (machine_id, state) VALUES ($1, $2) ON CONFLICT(machine_id) DO UPDATE set state=$2", &[&machine_id, &MachineState::Off]).await
            .map_err(|err| ModelHubServiceError(err.to_string()))?;

        tauri::async_runtime::spawn(async move {
            println!("[Paperspace] Invoking state poll");

            state_check_daemon(pool.clone(), mach, String::from("machine_start")).await;
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
    let db_client = PGClient::new()
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

    if results.trim().is_empty() {
        return Err(ModelHubServiceError("No data".to_owned()));
    };

    if results.contains("connection refused") {
        return Err(ModelHubServiceError("connection refused".to_string()));
    };

    match TrainingData::parse(&results) {
        Ok(training_data) => {
            insert_record(&training_data)
                .await
                .map_err(|err| ModelHubServiceError(format!("{:?}", err)))?;

            Ok(training_data)
        }

        Err(err) => Err(ModelHubServiceError(format!(
            "Parsing error failed {:?}",
            err
        ))),
    }
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

    let db_client = PGClient::new()
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
    pool: Pool,
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<(), ModelHubServiceError> {
    let machine_repo = MachineDb {
        client: pool.get().await.unwrap(),
    };

    let mut mach = machine_repo
        .get_machine_by(vec![( Queryable::Id, &machine_id)])
        .await
        .map_err(|err| ModelHubServiceError(err.into()))?;

    let mdl_hub_client = create_client(mach.provider.parse().unwrap()).unwrap();

    println!("[ModelHub]. Training on IP {:?}", mach.ip_address);

    // SSH into server to simply run 'train' command
    mdl_hub_client
        .train_model(mach.ip_address.unwrap())
        .await
        .map_err(|err| ModelHubServiceError(err.into()))?;

    mach.state = MachineState::Training;

    machine_repo
        .update_machine(mach)
        .await
        .map_err(|err| ModelHubServiceError(err.into()))?;

    let activity_log_db = ActivityLogDb {
        client: pool.get().await.unwrap(),
    };

    activity_log_db
        .create_activity_log(
            "starttrain".parse().unwrap(),
            &format!("{project_name} -> {deployment_name} -> {machine_id} has started training"),
        )
        .await
        .map_err(|err| ModelHubServiceError(err.into()))?;

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
