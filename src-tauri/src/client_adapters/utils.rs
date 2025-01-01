use std::{
    str::FromStr,
    fmt::Debug
};

use regex::Regex;

#[derive(Debug)]
pub struct ParseError(pub String);

pub fn parse_value_from_regex<T>(regex:Regex, text:&str) -> Result<T, ParseError>
where 
    T: FromStr,
    <T as FromStr>::Err:Debug{
    match regex.captures(&text){
        Some(val)=>{
            let res = val.get(1).ok_or(ParseError(format!("group not found for pattern '{:?}'", regex)));
            if let Err(err) = res{
                return Err(err)
            } 
            return Ok(
                res.unwrap().as_str()
                .to_owned()
                .parse::<T>()
                .map_err(|err|ParseError(format!("group found for {:?} but parse could not convert to expected type. {:?}", err, regex)))?
            )
        },
        None=> Err(ParseError(format!("Pattern not found for {:?}",regex )))
    }
}