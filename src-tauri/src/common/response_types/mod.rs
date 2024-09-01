use std::{ str, fmt};

use serde::Serialize;

pub mod project_responses;
pub mod config_response;

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

/// Uses serde_json crate to serialize responses into JSON response strings
/// `ResponseType` is the key areturns either {"data":<some_obj/type>} or {"error":<some obj/type>}
pub fn serialize_response<T: Serialize>( response_type:ResponseType, value:T) -> String{
    serde_json::to_string(&serde_json::json!({response_type.to_string():value})).unwrap()
}

/// Convenience wrappers around serializing a response with an 
/// error message. Do not send a serialized object to this function as this will
/// serialize again, the resialized object. Use `serialize_response`
/// instead
pub fn serialize_error<T: fmt::Display>(error:T) -> String{
    serialize_response(ResponseType::ERROR, error.to_string())
}

/// Convenience wrapper around serializing a response with a
/// success message, where response 'makes sense' as a string
/// Do not send a serialized object to this function as this will
/// serialize again, the resialized object. Use `serialize_response`
/// instead
pub fn serialize_success<T: fmt::Display>(value:T) -> String{
    serialize_response(ResponseType::DATA, value.to_string())
}
