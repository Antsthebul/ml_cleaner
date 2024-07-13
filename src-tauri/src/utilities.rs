use dotenvy;
use std::{
    env, 
    path, str, fmt};

pub enum ResponseType {
    DATA,
    ERROR
}
impl str::FromStr for ResponseType{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s{
            "data" => Ok(Self::DATA),
            "error"=> Ok(Self::ERROR),
            _=>Err(())
        }
    }
}

impl fmt::Display for ResponseType{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", match self{
            Self::DATA => "data",
            Self::ERROR=>"error"
        })
    }
}
pub fn load_env() -> Result<(), std::env::VarError>{
    
    dotenvy::from_path(path::Path::new("../.env"))
    .expect("env file shoud exist and have correct permissions");
 
    env::var("AWS_ACCESS_KEY")?;
    env::var("AWS_SECRET_KEY")?;
    env::var("PAPERSPACE_API_KEY")?;
    Ok(())
}

/// Uses serde_json crate to serialize responses into JSON response strings
/// `ResponseType` is the key areturns either {"data":<some_obj/type>} or {"error":<some obj/type>}
pub fn serialize_response<T: fmt::Display>( response_type:ResponseType, value:T) -> String{
    serde_json::to_string(&serde_json::json!({response_type.to_string():value.to_string()})).unwrap()
}

/// Convenience wrappers around serializing a response with an 
/// error message
pub fn serialize_error<T: fmt::Display>(error:T) -> String{
    serialize_response(ResponseType::ERROR, error)
}

///Convenience wrapper around serializing a response with a
/// success message
pub fn serialize_success<T: fmt::Display>(value:T) -> String{
    serialize_response(ResponseType::DATA, value)
}