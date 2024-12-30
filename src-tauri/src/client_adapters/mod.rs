pub mod database;
pub mod file_config;
pub mod image_verifier;
pub mod lake_client;
pub mod model_hub;
pub mod time_series_repo;
pub mod repository;
pub mod models;

use std::{fmt,env};


pub enum ENVIRONMENT{
    LOCAL,
    PRODUCTION
}

impl fmt::Display for ENVIRONMENT{
    fn fmt (&self, f: &mut fmt::Formatter<'_>)-> fmt::Result{
        match self{
            ENVIRONMENT::LOCAL => write!(f, "local"),
            ENVIRONMENT::PRODUCTION => write!(f, "production")
        }
    }
}

pub fn get_run_environment()-> ENVIRONMENT{
    match env::var("ENVIRONMENT"){
        Ok(val)=> {
            if val.to_lowercase() == String::from("local"){
            ENVIRONMENT::LOCAL
        }else{
            ENVIRONMENT::PRODUCTION
        }
        },
        Err(err)=>{
            println!("Environment should be set. {}", err);
            ENVIRONMENT::PRODUCTION
        }
    }
}