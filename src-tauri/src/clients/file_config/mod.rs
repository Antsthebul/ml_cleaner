use std::{collections::HashMap, fmt, fs, io::{self, prelude::*},net, path, str::FromStr};
use toml;
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub struct ConfigurationFileError(String);

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
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectMachine{
    id:String,
    name:String,
    machine_type:String,
    pub ip_addr: Option<net::Ipv4Addr>
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration{
    pub default_machine: Option<String>,
    pub projects:HashMap<String, Project>
    
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project{
    pub name: String,
    pub classes_file: Option<String>,
    pub info_file:Option<String>,
    pub machine: Option<ProjectMachine>
}

impl fmt::Display for ConfigurationFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}
impl Configuration {

    /// Loads a configuration file
    pub fn get_configuration_file() -> Result<Self, ConfigurationFileError>{
        let file_name = "../ml_cleaner.conf";
        let file = fs::File::open(file_name)
            .map_err(|err|ConfigurationFileError(err.to_string()))?;

        let mut buf_reader = io::BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();
        
        let configuration:Configuration = toml::from_str(&contents).map_err(|err|ConfigurationFileError(err.to_string()))?;
        
        Ok(configuration)
    }

    // Always overwrites entire file
    pub fn update_configuration_file(file:Configuration)-> Result<(), ConfigurationFileError>{
        let contents_str = toml::to_string(&file).map_err(|err|ConfigurationFileError(err.to_string()))?;
        fs::write("../ml_cleaner.conf", contents_str.as_bytes()).map_err(|err|ConfigurationFileError(err.to_string()))?;
        Ok(())

    }

    ///Returns complete list of projects
    /// and its related data
    pub fn get_all_projects() -> Result<HashMap<String, Project>, ConfigurationFileError>{
        let config = Configuration::get_configuration_file()?;
        Ok(config.projects)
    }

    pub fn get_project_by_project_name(name:&str) -> Result<Project, ConfigurationFileError>{
        let config = Configuration::get_configuration_file()?;
        
        match config.projects.get(&name.to_string()) {
            Some(project)=>Ok(project.clone()),
            None=>Err(ConfigurationFileError(format!("Configuration with name '{}' does not exist",name)))
        }
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

    ///Adds a project to configuration. Fails if a config
    /// exists with the same name
    pub fn add_project(&mut self, p:Project)-> Result<(), ConfigurationFileError>{
        match self.projects.get(&p.name){
            Some(_)=>Err(ConfigurationFileError(format!("{} already exists", p.name))),
            None=>{
                self.projects.insert(p.name.to_owned(), p);
                Ok(())
            }
        }
    }
}

/// Returns the file and bool indicating if the file was created, or returns 
/// error
pub fn create_file_if_not_present()-> Result<(),ConfigurationFileError>{
    let file_name = "../ml_cleaner.conf";
    match path::Path::new(file_name).try_exists(){
        Ok(true)=>{
            println!("{} configuration file exists", file_name );
            Ok(())
        },
        Ok(false)=>{
            match fs::File::create(file_name){
                Ok(_)=> {
                    let _= Configuration::update_configuration_file(Configuration{default_machine:Some("".to_string()),projects:HashMap::new()})?;
                    println!("{} configuration file was created", file_name);
                    Ok(())
                },
                Err(err)=> Err(ConfigurationFileError(err.to_string()))
            }
        },
        Err(err)=>Err(ConfigurationFileError(err.to_string()))
    }
}


