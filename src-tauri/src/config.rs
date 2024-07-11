use std::{fs, path, str::FromStr, io::{self, prelude::*}};
use toml;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ConfigurationFileError{
    pub message:String
}
pub enum ConfigurationKey{
    CLASSFILE,
    DEFAULTMACHINE
}
impl FromStr for ConfigurationKey{
    type Err = ();

    fn from_str(input: &str) -> Result<ConfigurationKey, Self::Err>{
        match input {
            "classes_file" => Ok(Self::CLASSFILE),
            "default_machine"=>Ok(Self::DEFAULTMACHINE),
            _=> Err(())
        }
    }
}
#[derive(Deserialize, Serialize)]
pub struct Configuration{
    default_machine: Option<String>,
    projects:Vec<Project>
    
}

#[derive(Deserialize, Serialize, Debug)]
struct Project{
    name: String,
    classes_file: Option<String>,
    info_file:Option<String>
}

impl Configuration {

    /// Loads a configuration file
    pub fn get_configuration_file() -> Result<Self, ConfigurationFileError>{
        let file_name = "../ml_cleaner.conf";
        let file = fs::File::open(file_name)
            .map_err(|err|ConfigurationFileError{message:err.to_string()})?;

        let mut buf_reader = io::BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        
        let configuration:Configuration = toml::from_str(&contents).map_err(|err|ConfigurationFileError{message:err.to_string()})?;
        
        Ok(configuration)
    }

    // Always overwrites entire file
    pub fn update_configuration_file(file:Configuration)-> Result<(), ConfigurationFileError>{
        let contents_str = toml::to_string(&file).map_err(|err|ConfigurationFileError{message:err.to_string()})?;
        fs::write("../ml_cleaner.conf", contents_str.as_bytes()).map_err(|err|ConfigurationFileError{message:err.to_string()})?;
        Ok(())

    }

    /// Sets the `machine_id` to the provide str slice
    /// if machine_id=`resetDefaultMachine`, the config
    /// will be set to an empty string
    pub fn update_machine_default(self, machine_id:&str){
        let value = match machine_id{

            "resetDefaultMachine"=>"default_machine =\"\"".to_string(),
            y=>format!("default_machine=\"{}\"", y)
        };
        fs::write("../ml_cleaner.conf",value.as_bytes()).unwrap();
    }

    /// `file_path` is the "key" path for the file in the Cloud environment 
    pub fn update_classes_file(self, file_path:&str){
        let value = match file_path {
            "resetClassesFile" => "classes_file=\"\"".to_string(),
            y=>format!("classes_file={}",y)
        };

        fs::write("../ml_cleaner.conf", value.as_bytes()).unwrap();
    }
}

/// Returns the file and bool indicating if the file was created, or returns 
/// error
pub fn create_file_if_not_present()-> Result<(),io::Error>{
    let file_name = "../ml_cleaner.conf";
    match path::Path::new(file_name).try_exists(){
        Ok(true)=>{
            println!("{} configuration file exists", file_name );
            Ok(())
        },
        Ok(false)=>{
            match fs::File::create(file_name){
                Ok(_)=> {
                    println!("{} configuration file was created", file_name);
                    Ok(())
                },
                Err(err)=> return Err(err)
            }
        },
        Err(err)=>Err(err)
    }
}


