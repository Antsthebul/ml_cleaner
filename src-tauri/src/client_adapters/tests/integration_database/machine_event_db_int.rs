use crate::client_adapters::{database::machine_event_db::MachineEventDb, model_hub::MachineState};

use super::MockDbClient;

#[tokio::test]
async fn test_create_machine_event() {
    let client = MockDbClient::new().await.unwrap();
    let test_machine_id = "ID12345";

    let _ = client.execute("DELETE FROM machine_events", &[]).await.unwrap();
    let _ = client.execute("DELETE FROM machines", &[]).await.unwrap();

    let _ = client
        .execute(
            "INSERT INTO machines (machine_id, model, price, state, provider) 
            VALUES ($1, 'testModel', 0.03, $2, 'paperspace')",
            &[&test_machine_id, &MachineState::Off],
        )
        .await
        .unwrap();

    let machine_event_repo = MachineEventDb { client };

    machine_event_repo
        .create_machine_event("start".parse().unwrap(), test_machine_id)
        .await
        .unwrap();
}
