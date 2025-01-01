use super::machine_db::MachineCreate;

#[test]
fn parse_machine_create_from_str(){
    let model = "A4000";
    let data = serde_json::json!({
        "machine_id":"Test1",
        "model":model,
        "price":0.03,
    });

    let ser_data = serde_json::to_string(&data).unwrap();

    let res = MachineCreate::from_ser_map(&ser_data).unwrap();

    assert_eq!(res.model, model)
}