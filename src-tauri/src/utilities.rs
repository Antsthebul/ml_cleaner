use dotenvy;
use std::{
    env, 
    path};

pub fn load_env() -> Result<(), std::env::VarError>{
    
    dotenvy::from_path(path::Path::new("../.env"))
    .expect("env file shoud exist and have correct permissions");
 
    env::var("AWS_ACCESS_KEY")?;
    env::var("AWS_SECRET_KEY")?;
    env::var("PAPERSPACE_API_KEY")?;
    Ok(())
}