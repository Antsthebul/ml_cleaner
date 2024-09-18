use std::{collections::HashMap, fs, io::{self, BufReader}, path};

use serde_json::json;


/// Creates local cache dir with state file
pub fn create_cache() -> io::Result<()>{

    if !path::Path::new(".cache/").exists(){

        fs::create_dir(".cache/").expect("Cannot create local dir");

        let value = serde_json::to_string(&json!({"app":"Orkestr8"})).unwrap();
        
        fs::write(".cache/state.json", value.as_bytes()).expect("cache file cannot be created")
    }else{
        if !path::Path::new(".cache/state.json").exists(){
            let value = serde_json::to_string(&json!({"app":"Orkestr8"})).unwrap();
        
            fs::write(".cache/state.json", value.as_bytes()).expect("cache file cannot be created")
    
        }
    }

    Ok(())
}
/// Updates local cache with rpovided value. .Cache is a JSON file
/// Where each aside from app name, each key is a volatile entity
/// and the value 'schema' as a string is <value>;<lastUpdateUTC>;
pub fn update_cache(key:&str, value: &str) -> io::Result<()>{
    let file = fs::File::open(".cache/state.json")?;
    let reader = BufReader::new(file);
    let data: HashMap<String, serde_json::Value> = serde_json::from_reader(reader).unwrap();

    let mut state_map:HashMap<&str, String> = HashMap::new();
    
    for k in data.keys(){
        let val = data.get(k).unwrap().to_string();
        state_map.insert(k, val);
    }
    let date = chrono::offset::Utc::now();
    let built_val = format!("{};{}", value, date);
    state_map.insert(key, built_val);

    let contents = serde_json::to_string(&state_map).unwrap();    
    let _ = fs::write(".cache/state.json", contents.as_bytes())?;
    Ok(())
}

