use dotenv;
use std::env;

pub fn load_env() -> Result<(), std::env::VarError>{
    dotenv::dotenv().ok();
    env::var("AWS_ACCESS_KEY")?;
    env::var("AWS_SCRET_KEY")?;
    Ok(())
}