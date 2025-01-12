use deadpool_postgres::Object;
use postgres::Row;

use super::DbClientError;

pub struct DeploymentCreate{
    pub name: String,
    pub project_id: i32,
    pub tags: Vec<String>,
    pub description: String

}

pub struct Deployment{
    pub id: i32,
    pub name: String,
    pub project_id: i32,
    pub tags: Vec<String>,
    pub description: String

}
impl Deployment {
    pub fn from_row(row:Row) -> Self{
        Self { 
            id: row.get("id"),
            name: row.get("name"), 
            project_id: row.get("project_id"), 
            tags: vec![], 
            description: row.get("description") }
    }
}

pub struct DeplyomentDb{
    pub client: Object
}

impl DeplyomentDb{
    pub async fn create_deployment(&self, data: DeploymentCreate) -> Result<(), DbClientError>{
        let _ = self.client
            .execute("INSERT INTO deployments (name, project_id, description) VALUES ($1,$2, $3)"
                , &[&data.name, &data.project_id, &data.description])
            .await
            .map_err(|err|DbClientError(format!("unable tocreate deployment. {err}" )))?;

        Ok(())
    }

    pub async fn add_tags_to_deployment(&self, 
        deployment_id: &i32, 
        tags: Vec<String>) -> Result<(), DbClientError>{
            let _ = self.client
                .execute("INSERT INTO tags (key, name, table) VALUES($1, $2, $3)", &[&deployment_id])
                .await
                .map_err(|err|DbClientError(format!("tags '{:?}' unable to be added. {err}", tags)))?;
        
            Ok(())
    }

    pub async fn get_deployment_by_name(&self, deployment_name:&str) -> Result<Deployment ,DbClientError>{
        let row = self.client
            .query_one("SELECT * FROM deployments WHERE name=$1", &[&deployment_name])
            .await
            .map_err(|err|DbClientError(format!("unable to retrieve deployment. {err}")))?;
        
        Ok(Deployment::from_row(row))
    }
}