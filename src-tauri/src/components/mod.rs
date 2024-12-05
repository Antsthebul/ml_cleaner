/// Components is the core of the library crate. This is all the
/// funcdemental logic for what 'components' to use and should be
/// exapnded upon. For example, a repository (default: file)
/// a ML workspace (default:paperspace), and a ML data store(AWS)
/// will make up the usable commponents. For web APIS, CLIs, or desktop
/// app, which is controlled by the binary crate.
pub mod adapters;
pub mod repository;


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