
#[tauri::command]
pub async fn get_machine_status(project_name:&str,machine_id:&str)-> Result<String, String>{
    let pc = PaperSpaceClient::new();

    let machine = pc.get_machine_status(project_name, machine_id).await
        .map_err(|err|serialize_error(err.to_string()))?;

    let response = serde_json::json!({"data":machine});
    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn list_machines()-> Result<String, String>{
    let pc = PaperSpaceClient::new();
    let machines = pc.get_machines().await
    .map_err(|err|serialize_error(err.to_string()))?;

    let response = serde_json::json!({"data":machines});
    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn start_machine(machine_id:&str) -> Result<String, String>{
    let pc = PaperSpaceClient::new();
    let  _ = pc.handle_machine_run_state(machine_id, "start").await
        .map_err(|err|serde_json::to_string(&serde_json::json!({"error":err.to_string()})).unwrap())?;

    let response = serde_json::json!({"data":"success"});

    Ok(serde_json::to_string(&response).unwrap())
}

#[tauri::command]
pub async fn stop_machine(machine_id:&str) -> Result<String, String>{
    let pc = PaperSpaceClient::new();
    pc.handle_machine_run_state(machine_id, "stop").await
        .map_err(|err|serialize_error(err))?;

    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn train_model(ip_address:&str)-> Result<String, String>{
    let pc = PaperSpaceClient::new();
    let _ = pc.train_model(ip_address.parse().unwrap()).await
    .map_err(|err|serialize_error(err))?;

    Ok(serialize_success("success"))
}

#[tauri::command]
pub async fn get_machine_by_machine_id(machine_id:&str) -> Result<String, String>{
    let pc = PaperSpaceClient::new();
    let result = pc.get_machine_by_machine_id(machine_id).await
        .map_err(|err|serialize_success(err))?;
    Ok(serialize_success(result))
}