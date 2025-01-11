use crate::client_adapters::database::machine_db::{MachineCreate, MachineDb};
use super::MockDbClient;



#[tokio::test]
async fn test_machine_can_be_created_retrieved_and_deleted_success(){
    let machine_id = "Test1";
    // ARRANGE
    let mc = MachineCreate{
        machine_id: machine_id.into(),
        model:"A4000".into(),
        price:0.04,
        provider:"paperspace".into()
    };

    let client = MockDbClient::new().await.unwrap();

    let _ = client.execute("DELETE FROM machines", &[])
        .await.unwrap();
    
    let machine_db = MachineDb{client};

    // ACT - CREATE
    let _ = machine_db.create_machine(mc).await.unwrap();
    
    
    // ACT - RETRIEVE
    let machine = machine_db.get_machine_by_id(machine_id)
        .await.unwrap();

    // ASSERT
    assert_eq!(machine.machine_id, machine_id);

    // ACT - DELETE
    let _ = machine_db.delete_machine(machine_id).await.unwrap();

    // ASSERT - DELETE
    let error_msg = machine_db.get_machine_by_id(machine_id)
    .await.err();

    if let Some(msg) = error_msg  {
        
        assert!(msg.0.contains("not found"))
    }else{
        panic!("test failed")
    }
}