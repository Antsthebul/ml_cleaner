use dotenvy;
use std::{
    env, 
    path};

pub fn load_env() -> Result<(), std::env::VarError>{
    
    match dotenvy::from_path(path::Path::new("../.env")){
        Ok(_)=>(),
        Err(err)=>println!("Your error {}", err)
    };
 
    env::var("AWS_ACCESS_KEY")?;
    env::var("AWS_SECRET_KEY")?;
    env::var("PAPERSPACE_API_KEY")?;
    Ok(())
}