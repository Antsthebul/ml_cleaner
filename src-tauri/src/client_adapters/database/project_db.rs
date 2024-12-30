

use std::collections::HashMap;

use tokio_postgres::Client;

use crate::client_adapters::models::{Deployment, Project};

use super::DbClientError;

// let client = PGClient::new().await
// .map_err(|err| DbClientError(format!("environment_db could not create client to get all projects. {}", err))
// )?;

pub struct ProjectDb{
    pub client: Client
}

impl ProjectDb{

    pub async fn get_all_projects(&self) -> Result<Vec<Project>, DbClientError>{

        let rows = self.client.query("SELECT p.id as project_id, p.name as project_name, 
        p.created_at as project_created_at, d.id as deployment_id, d.name as deployment_name, 
        d.created_at as deployment_created_at FROM projects p 
        JOIN deployments d ON p.id = d.project_id", &[]).await
            .map_err(|err| DbClientError(format!("environment_db failed when getting all projects. {}", err))
            )?;
                   
        let mut projects: HashMap<i32, Project> = HashMap::new();
        
        for r in rows{
            let cur_project = projects.get_mut(&r.get::<_, i32>("project_id"));
            
            let deployment = Deployment { 
                name: r.get("deployment_name"), 
                machines:vec![],
                created_at:r.get("deployment_created_at") 
            };

            if let Some(project) = cur_project{
                project.add_deployment(deployment);

            }else{
                let idx:i32 = r.get("project_id");
                let project =Project{
                    id:idx,
                    name: r.get("project_name"),
                    deployments:vec![deployment],
                    created_at: r.get("project_created_at")
                };
                projects.insert(idx, project);
            }
        }
        
        Ok(projects.into_values().collect())
    }

    async fn get_project_by_name(&self, name:&str, )->Result<Project, DbClientError>{
       

        let rows = self.client.query("SELECT p.id as project_id, p.name as project_name, 
        p.created_at as projects_created, d.id as deployment_id, d.name as deployment_name, 
        d.created_at as deployment_created_at FROM projects p 
        JOIN deployments d ON p.id = d.project_id
        WHERE d.name =$1", &[&name]).await
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
}