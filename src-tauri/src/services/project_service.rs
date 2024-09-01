use app::{file_config::{Project,Configuration},
    // get_classes_data
    };
use crate::common::response_types::{ serialize_error, serialize_response};

#[derive(Debug)]
pub struct ProjectError(String);

impl std::fmt::Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f, "{}", self.0)
    }
}

pub async fn get_all_projects()->Result<Vec<Project>, ProjectError>{
    let config = Configuration::get_all_projects()
        .map_err(|err|ProjectError(err.to_string()))?;
    
    Ok(config.iter().map(|c|c.to_owned()).collect())
    
}

/// Returns serialized Result or Error. The serialized result is
/// a project with other additional metadata.
pub async fn get_project_deployment(project_name:&str, deploy_name:&str) -> Result<String, ProjectError>{
    let project = Configuration::get_project_by_project_name(project_name)
    .map_err(|err|ProjectError(err.to_string()))?;

    let deployment = project.get_project_deployment(deploy_name).unwrap();

    // We want to either get the file(s) or generate them
    // for now fail fast and return single errors, not lists
    if let Some(files) = deployment.files{
        for f in vec!["train", "test"]{

            if let None = files.get(f){
                return Err(ProjectError(String::from(format!("No {:?} file found",f))))
            }
        }
    };
    // let file_path = match &deployment.classes_file{
    //     Some(file) => file,
    //     // Return bare 'inititalized' 
    //     None=>{
    //         let res = serde_json::json!({"deployment":deployment, "classes_data":Vec::<String>::new()});
    //         return Err(serialize_response("data".parse().unwrap(), res))}
    // };
    // // Add meta data. No need for 'response' struct
    // let class_data = get_classes_data(file_path).await
    // .map_err(|err|serialize_error(err.to_string()))?;
    // let response = serde_json::json!({"deployment":deployment, "classes_data":class_data});

    Ok(serialize_response("data".parse().unwrap(), "success"))
}

pub async fn get_project_by_project_name(project_name:&str)-> Result<Project, ProjectError>{

    Ok(Configuration::get_project_by_project_name(project_name)
        .map_err(|err|ProjectError(err.to_string()))?)
}
