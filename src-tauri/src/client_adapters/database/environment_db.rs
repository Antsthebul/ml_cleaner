use crate::client_adapters::models::{Deployment, Project};

use super::{DbClient, DbClientError};



pub async fn get_all_projects() -> Result<Vec<Project>, DbClientError>{
    let client = DbClient::new().await
        .map_err(|err| DbClientError(format!("environment_db could not create client to get all projects. {}", err))
        )?;

    let rows = client.query("SELECT p.id as project_id, p.name as project_name, 
        p.created_at as projects_created, d.id as deployment_id, d.name as deployment_name, 
        d.created_at as deployment_created_at FROM projects p 
        JOIN deployments d ON p.id = d.project_id", &[]).await
        .map_err(|err| DbClientError(format!("environment_db failed when getting all projects. {}", err))
        )?;

    let mut results:Vec<Project> = vec![];
    let mut deployments: Vec<Deployment> = vec![];

    let mut cur_project_id:Option<i32> = None;

    for r in rows{

        if let Some(project_id) = cur_project_id{
            let deployment = Deployment { 
                name: r.get("deployment_name"), 
                machines:vec![],
                created_at:r.get("deployment_created_at") 
            };
            if project_id == r.get::<&str, i32>("project_id"){
                deployments.push(deployment);
            }else{
                
                results.push(Project{
                    id:r.get("project_id"),
                    name: r.get("name"),
                    deployments:deployments,
                    created_at: r.get("created_at")
                });
                deployments = vec![];
            }
        }else{
            cur_project_id = Some(r.get("project_id"))
        }

    }

    Ok(results)
}

async fn get_project_by_name(name:&str)->Result<Project, DbClientError>{
    let client = DbClient::new().await
        .map_err(|err| DbClientError(format!("environment_db could not create client to get all projects. {}", err))
        )?;

    let rows = client.query("SELECT p.id as project_id, p.name as project_name, p.created_at as projects_created, d.id as deployment_id, d.name as deployment_name, d.created_at as deployment_created_at FROM projects p JOIN deployments d ON p.id = d.project_id", &[]).await
        .map_err(|err| DbClientError(format!("environment_db failed when getting all projects. {}", err))
        )?;

    let mut deployments = vec![];
    for r in &rows{
        deployments.push(
            Deployment{
                name:r.get("name"),
                created_at: r.get("created_at"),
                machines:vec![]
            }
        )        
    }

    Ok(Project{
        id:rows[0].get("id"),
        name:rows[0].get("name"),
        created_at:rows[0].get("created_at"),
        deployments:deployments

    })

   

}