use tauri::State;

use crate::{
    app::{
        common::response_types::{serialize_error, serialize_response, serialize_success},
        services::model_hub_service,
    },
    AppState,
};

#[tauri::command]
pub async fn generate_test_train_data(project_name: &str) -> Result<String, String> {
    let train_pct = Some(0.8);

    let _ = model_hub_service::generate_test_train_data(project_name, train_pct)
        .await
        .map_err(|err| serialize_error(err.to_string()));

    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn get_machine_status(
    deployment_name: &str,
    project_name: &str,
) -> Result<String, String> {
    // A rather noisy log
    // println!("[ModelHubCommand] - GET Machine Status request recieved for deployment '{}' in project '{}'", deployment_name, project_name);

    let machines = model_hub_service::get_machine_status(deployment_name, project_name)
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    let mut values = Vec::new();

    for m in &machines {
        let tmp = serde_json::json!(
        {
            "id":m.id,
            "ip_address":m.ip_address,
            "state":m.state.to_string()}
        );

        values.push(tmp);
    }
    Ok(serialize_response("data".parse().unwrap(), values))
}

#[tauri::command]
pub async fn list_machines() -> Result<String, String> {
    // let pc = PaperSpaceClient::new();
    // let machines = pc.get_machines().await
    // .map_err(|err|serialize_error(err.to_string()))?;

    // let response = serde_json::json!({"data":machines});
    // Ok(serde_json::to_string(&response).unwrap())
    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn start_machine(
    state: State<'_, AppState>,
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<String, String> {
    println!(
        "[ModelHubStartCommand] - START model request recieved for machine_id '{}'",
        machine_id
    );

    let _ = model_hub_service::start_or_stop_machine(
        state.pool.clone(),
        deployment_name,
        project_name,
        machine_id,
        "start",
    )
    .await
    .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn stop_machine(
    state: State<'_, AppState>,
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<String, String> {
    println!(
        "[ModelHubStopCommand] - STOP model request recieved for machine_id '{}'",
        machine_id
    );

    let _ = model_hub_service::start_or_stop_machine(
        state.pool.clone(),
        deployment_name,
        project_name,
        machine_id,
        "stop",
    )
    .await
    .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn train_model(
    state: State<'_, AppState>,
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<String, String> {
    println!(
        "[ModelHubTrainCommand] - Training model request recieved for deployment '{}'",
        deployment_name
    );

    let _ = model_hub_service::train_model(
        state.pool.clone(),
        deployment_name,
        project_name,
        machine_id,
    )
    .await
    .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn stop_train_model(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<String, String> {
    println!(
        "[ModelHubCommand] - Stop Training model request recieved for deyploment '{}'",
        deployment_name
    );
    let _ = model_hub_service::stop_train_model(deployment_name, project_name, machine_id)
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}

// #[tauri::command]
// pub async fn get_machine_by_machine_id(machine_id: &str) -> Result<String, String> {
//     // let pc = PaperSpaceClient::new();
//     // let result = pc.get_machine_by_machine_id(machine_id).await
//     //     .map_err(|err|serialize_success(err))?;
//     Ok(serialize_success("success"))
// }

#[tauri::command]
pub async fn get_training_results(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<String, String> {
    println!(
        "[MODEL HUB COMMAND] - Poll for training results for '{}'",
        deployment_name
    );

    let data = model_hub_service::get_training_results(deployment_name, project_name, machine_id)
        .await
        .map_err(|err| serialize_error(err.to_string()))?;
    let res = serialize_response("data".parse().unwrap(), data);
    println!("sending {:?}", res);
    Ok(res)
}

#[tauri::command]
pub async fn download_model(
    deployment_name: &str,
    project_name: &str,
    machine_id: &str,
) -> Result<String, String> {
    println!(
        "[MODEL HUB COMMAND] - Transfering model request for {}",
        deployment_name
    );

    let _ = model_hub_service::download_model(deployment_name, project_name, machine_id)
        .await
        .map_err(|err| serialize_error(err.to_string()))?;

    Ok(serialize_success("success"))
}
