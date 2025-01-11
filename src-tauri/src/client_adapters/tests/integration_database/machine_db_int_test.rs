use super::MockDbClient;
use crate::client_adapters::database::machine_db::{MachineCreate, MachineDb, Queryable};

#[tokio::test]
async fn test_machine_can_be_created_retrieved_and_deleted_success() {
    let machine_id = "Test1";
    // ARRANGE
    let mc = MachineCreate {
        machine_id: machine_id.into(),
        model: "A4000".into(),
        price: 0.04,
        provider: "paperspace".into(),
        project_id: None,
        deployment_id: None
    };

    let client = MockDbClient::new().await.unwrap();

    let _ = client.execute("DELETE FROM machine_events", &[]).await.unwrap();
    let _ = client.execute("DELETE FROM machines", &[]).await.unwrap();

    let machine_db = MachineDb { client };

    // ACT - CREATE
    machine_db.create_machine(mc).await.unwrap();

    // ACT - RETRIEVE
    let machine = machine_db.get_machine_by(vec![( Queryable::Id, &machine_id)])
        .await.unwrap();

    // ASSERT
    assert_eq!(machine.machine_id, machine_id);

    // ACT - DELETE
    machine_db.delete_machine(machine_id).await.unwrap();

    // ASSERT - DELETE
    let error_msg = machine_db.get_machine_by(vec![( Queryable::Id, &machine_id)]).await.err();

    if let Some(msg) = error_msg {
        assert!(msg.0.contains("query returned an unexpected number of rows"))
    } else {
        panic!("test failed")
    }
}
