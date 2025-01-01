use std::{
    sync::Arc,
    collections::HashMap};

use tokio_postgres::Client;

use crate::client_adapters::models::{Deployment, Project};

use super::DbClientError;


pub struct ProjectDb{
    pub client: Arc<Client>
}

impl ProjectDb{

    pub async fn get_project_id(&self, project_name:&str) -> Result<i32, DbClientError>{
        let rows = self.client.query("SELECT id FROM projects WHERE name = $1", &[&project_name])
        .await.map_err(|err| DbClientError(format!("unable to get project id use name={}. {}",project_name, err)))?;

        if let Some(row) = rows.first(){

            Ok(row.get("id"))
        }else{
            Err(DbClientError("project not found".into()))
        }


    }

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

    pub async fn get_project_by_name(&self, name:&str, )->Result<Project, DbClientError>{
       
        let rows = self.client.query("SELECT p.id as project_id, p.name as project_name, 
            p.created_at as project_created_at, d.id as deployment_id, d.name as deployment_name, 
            d.created_at as deployment_created_at FROM projects AS p 
            LEFT OUTER JOIN deployments d ON p.id = d.project_id WHERE p.name = $1", &[&name]).await
                .map_err(|err| DbClientError(format!("environment_db failed when getting all projects. {}", err))
        )?;
        
        
        if rows.len() == 0 {
            return Err(DbClientError(format!("get project by name using name={name} returned 0 rows.")))
        }
        let mut deployments = vec![];


        for r in &rows{
            if let Some(name) = r.get("deployment_name"){

                deployments.push(
                    
                    Deployment{
                        name:name,
                        created_at: r.get("deployment_created_at"),
                        machines:vec![]
                    }
                )        
            }
        }
        
        Ok(Project{
            id:rows[0].get("project_id"),
            name:rows[0].get("project_name"),
            created_at:rows[0].get("project_created_at"),
            deployments:deployments
            
        })
    }

    pub async fn get_project_deployment_by_name(&self,project:&str, name:&str)-> Result<Deployment,DbClientError>{
        let rows = self.client
            .query("SELECT d.* from deployments d 
                JOIN projects p ON p.id = d.project_id 
                WHERE d.name=$2 AND p.name=$1", &[&project, &name])
            .await.map_err(|err|DbClientError(
                format!("deployment db failed to retrieve deployment using name='{}'. {}",name,err)))?;
        if let Some(row) = rows.first(){

            Ok(Deployment{
                name:row.get("name"),
                machines:vec![],
                created_at:row.get("created_at")
            })
        }else{
            Err(DbClientError(format!("failed to retrieve deployment from project={project} using name={name}. No rows found")))
        }
    }

    /// Updates a project if found else creates.
    pub async fn upsert_project(&self, name: &str)-> Result<(), DbClientError>{
        let _ = self.client.execute( "INSERT INTO projects (name) VALUES ($1)
            ON CONFLICT(name) 
            DO UPDATE SET name=$1", &[&name])
            .await.map_err(|err| DbClientError(format!("unable to create project using name={name}. {}", err)))?;
        
        Ok(())

    }

    /// Updates a deployment if found, else creates
    pub async fn upsert_deployment(&self, project_name:&str, deployment_name:&str) -> Result<Deployment, DbClientError>{
        let project_id = self.get_project_id(project_name)
            .await.map_err(|err| DbClientError(format!("unable to create deployment. {}", err)))?;

        let _  = self.client.execute("INSERT INTO deployments (project_id, name) VALUES ($1, $2)", &[&project_id, &deployment_name])
            .await.map_err(|err| DbClientError(format!("unable to create deployment. {}", err)))?;

        Ok(
            self.get_project_deployment_by_name(project_name, &deployment_name)
                .await.map_err(|err| DbClientError(format!("unable to return new deployment. {}", err)))?

        )
    }

    pub  async fn delete_deployment(&self, project_name:&str, deployment_name:&str) -> Result<(), DbClientError>{
        let project_id = self.get_project_id(project_name)
            .await.map_err(|err| DbClientError(format!("delete deployment failed. project={}, deployment_name={}. {}",project_name, deployment_name, err)))?;

        
        let _ = self.client.execute("DELETE FROM deployments WHERE name=$1 AND project_id=$2", &[&deployment_name, &project_id])
            .await.map_err(|err| DbClientError(format!("unable to delete deployment 
            using  project={}, deployment_name={}. {}",project_name, deployment_name, err)))?;

        Ok(())
    }
}