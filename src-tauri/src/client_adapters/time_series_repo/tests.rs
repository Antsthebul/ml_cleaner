use super ::*;

#[test]
fn test_parse_log_record_successfully_parses(){
    let text = "[Data-row] epoch=10, train_acc=56%, 
        test_acc=71%, time=30.50sec, 
        train_loss=1.2345, val_loss=2.3456, dir_name=wack";
 
    let data = TrainingData::parse(text).unwrap();

    assert_eq!(data.epoch, 10);
    assert_eq!(data.dir_name, "wack".to_string());
    assert_eq!(data.duration, 30.50);
    assert_eq!(data.train_loss, 1.2345);
    
}