use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fmt, fs,
    io::{self, prelude::*},
    net::{self, Ipv4Addr},
    path,
    str::FromStr,
};
use toml;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Repository {
    pub provider: String,
    pub storage_type: String,
    pub path: String,
    pub name: String,
}

#[derive(Debug)]
pub struct ConfigurationFileError(pub String);

pub enum ConfigurationKey {
    CLASSFILE,
    DEFAULTMACHINE,
}
impl FromStr for ConfigurationKey {
    type Err = ();

    fn from_str(input: &str) -> Result<ConfigurationKey, Self::Err> {
        match input {
            "classes_file" => Ok(Self::CLASSFILE),
            "default_machine" => Ok(Self::DEFAULTMACHINE),
            _ => Err(()),
        }
    }
}
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProjectMachine {
    pub provider: String,
    pub id: String,
    name: String,
    machine_type: String,
    pub ip_addr: Option<net::Ipv4Addr>,
}
/// Root of configuration
#[derive(Deserialize, Serialize, Debug)]
pub struct Configuration {
    pub default_machine: Option<String>,
    pub projects: Vec<Project>,
}
/// A specific environment that relates to the project
/// Examples like Dev/Stg/Prod  for an CNN project
/// would be dscribed as a deployment
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Deployment {
    pub name: String,
    pub machines: Vec<ProjectMachine>,
    pub created_at: NaiveDateTime
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Project {
    pub id:i32,
    pub name: String,
    pub deployments: Vec<Deployment>,
    pub created_at: NaiveDateTime
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct FileAttr {
    pub path: String,
    pub exists: bool,
}
impl Project {
    pub fn get_project_deployment(
        &self,
        deploy_name: &str,
    ) -> Result<Deployment, ConfigurationFileError> {
        match self.deployments.iter().find(|&dp| dp.name == deploy_name) {
            Some(environ) => Ok(environ.to_owned()),
            None => Err(ConfigurationFileError(format!(
                "'{deploy_name}' does not exist for {}",
                self.name
            ))),
        }
    }
}
impl Deployment {
    pub fn get_machine_by_machine_id(
        self,
        machine_id: &str,
    ) -> Result<ProjectMachine, ConfigurationFileError> {
        for m in self.machines {
            if m.id == machine_id {
                return Ok(m);
            }
        }
        Err(ConfigurationFileError(format!(
            "Machine ID '{}' not found",
            machine_id
        )))
    }
    /// Reteives a Machine using the Machines IPv4 Addr
    pub fn get_machine_by_ip(
        self,
        ip_address: Ipv4Addr,
    ) -> Result<ProjectMachine, ConfigurationFileError> {
        for m in self.machines {
            if let Some(ip_addr) = m.ip_addr {
                if ip_addr == ip_address {
                    return Ok(m);
                }
            }
        }
        Err(ConfigurationFileError(format!(
            "Machine IP addr '{}' not found",
            ip_address
        )))
    }
}

impl fmt::Display for ConfigurationFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl Configuration {
    /// Loads a configuration file
    pub fn get_configuration_file() -> Result<Self, ConfigurationFileError> {
        let file_name = "../ml_cleaner.conf";
        let file =
            fs::File::open(file_name).map_err(|err| ConfigurationFileError(err.to_string()))?;

        let mut buf_reader = io::BufReader::new(file);
        let mut contents = String::new();
        buf_reader.read_to_string(&mut contents).unwrap();

        let configuration: Configuration =
            toml::from_str(&contents).map_err(|err| ConfigurationFileError(err.to_string()))?;

        Ok(configuration)
    }

    // Always overwrites entire file
    pub fn update_configuration_file(file: Configuration) -> Result<(), ConfigurationFileError> {
        let contents_str =
            toml::to_string(&file).map_err(|err| ConfigurationFileError(err.to_string()))?;
        fs::write("../ml_cleaner.conf", contents_str.as_bytes())
            .map_err(|err| ConfigurationFileError(err.to_string()))?;
        Ok(())
    }

    ///Returns complete list of projects
    /// and its related data
    pub fn get_all_projects() -> Result<Vec<Project>, ConfigurationFileError> {
        let config = Configuration::get_configuration_file()?;
        Ok(config.projects)
    }

    pub fn get_project_by_project_name(
        project_name: &str,
    ) -> Result<Project, ConfigurationFileError> {
        let config = Configuration::get_configuration_file()?;

        match config
            .projects
            .iter()
            .find(|&proj| project_name == &proj.name)
        {
            Some(project) => Ok(project.clone()),
            None => Err(ConfigurationFileError(format!(
                "Configuration with name '{}' does not exist",
                project_name
            ))),
        }
    }

    /// Sets the `machine_id` to the provide str slice
    /// if machine_id=`resetDefaultMachine`, the config
    /// will be set to an empty string
    pub fn update_machine_default(self, machine_id: &str) {
        let value = match machine_id {
            "resetDefaultMachine" => "default_machine =\"\"".to_string(),
            y => format!("default_machine=\"{}\"", y),
        };
        fs::write("../ml_cleaner.conf", value.as_bytes()).unwrap();
    }

    /// `file_path` is the "key" path for the file in the Cloud environment
    pub fn update_classes_file(self, file_path: &str) {
        let value = match file_path {
            "resetClassesFile" => "classes_file=\"\"".to_string(),
            y => format!("classes_file={}", y),
        };

        fs::write("../ml_cleaner.conf", value.as_bytes()).unwrap();
    }

    /// Adds a project to configuration. Fails if a config
    /// exists with the same name
    pub fn add_project(&mut self, p: Project) -> Result<(), ConfigurationFileError> {
        match self.projects.iter().find(|&proj| proj.name == p.name) {
            Some(_) => Err(ConfigurationFileError(format!("{} already exists", p.name))),
            None => {
                self.projects.push(p);
                Ok(())
            }
        }
    }
}