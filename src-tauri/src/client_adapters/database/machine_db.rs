use std::{fmt::Display, net::Ipv4Addr};

use deadpool_postgres::Object;
use postgres::Row;
use postgres_types::ToSql;

use crate::client_adapters::{model_hub::MachineState, utils::ParseError};

use super::{sq_builder::SQBuilder, DbClientError};
use serde::Deserialize;

pub enum Queryable{
    Id,
    ProjectId,
    DeploymentId
}

impl Display for Queryable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self{
            Self::DeploymentId => "deployment_id",
            Self::Id=> "machine_id",
            Self::ProjectId => "project_id"
        };
        
        write!(f, "{output}")
    }
}

#[derive(Deserialize)]
pub struct MachineCreate {
    pub machine_id: String,
    pub model: String,
    pub price: f32,
    pub provider: String,
    pub deployment_id: Option<i32>,
    pub project_id: Option<i32>
}

#[derive(Deserialize)]
pub struct Machine {
    pub machine_id: String,
    pub model: String,
    pub price: f32,
    pub state: MachineState,
    pub ip_address: Option<Ipv4Addr>,
    pub provider: String,
    pub deployment_id: Option<i32>,
    pub project_id: Option<i32>    
}

impl Machine {
    pub fn from_ser_map(data_str: &str) -> Result<Machine, ParseError> {
        Ok(serde_json::from_str::<Machine>(data_str)
            .map_err(|err| ParseError(format!("unable to parse from_str. {err}")))?)
    }

    pub fn from_row(row: Row) -> Result<Self, ParseError> {
        let mut ip_address = None;
        if let Some(ip_addr) = row.get::<_, Option<String>>("ip_address") {
            if ip_addr != "".to_string() {
                ip_address = ip_addr.parse().ok();
                if None == ip_address {
                    return Err(ParseError(format!(
                        "ip_address could not be parsed from {ip_addr}"
                    )));
                }
            }
        };
        Ok(Self {
            machine_id: row.get("machine_id"),
            model: row.get("model"),
            price: row.get("price"),
            state: row.get("state"),
            ip_address: ip_address,
            provider: row.get("provider"),
            project_id: row.get("project_id"),
            deployment_id: row.get("deployment_id")
        })
    }
}

impl MachineCreate {
    /// `data_str` MUST be in the form of serialized mapping
    pub fn from_ser_map(data_str: &str) -> Result<MachineCreate, ParseError> {
        Ok(serde_json::from_str::<MachineCreate>(data_str)
            .map_err(|err| ParseError(format!("unable to parse from_str. {err}")))?)
    }
}

pub struct MachineDb {
    pub client: Object,
}

impl MachineDb {
    pub async fn create_machine(&self, data: MachineCreate) -> Result<(), DbClientError> {
        let _ = self
            .client
            .execute(
                "INSERT INTO machines 
            (machine_id, model, price, state, provider) VALUES ($1,$2,$3, $4, $5)",
                &[
                    &data.machine_id,
                    &data.model,
                    &data.price,
                    &MachineState::Off,
                    &data.provider,
                ],
            )
            .await
            .map_err(|err| DbClientError(format!("unable to create machine {err}")))?;
        Ok(())
    }


    /// Operates like a 'PUT' action to update `machines`
    pub async fn update_machine(&self, data: Machine) -> Result<(), DbClientError> {
        let _ = self
            .client
            .execute(
                "UPDATE machines 
            SET model=$2, price=$3, state=$4
            WHERE machine_id=$1",
                &[&data.machine_id, &data.model, &data.price, &data.state],
            )
            .await
            .map_err(|err| DbClientError(format!("unable to create machine {err}")))?;

        Ok(())
    }

    pub async fn delete_machine(&self, machine_id: &str) -> Result<(), DbClientError> {
        let _ = self
            .client
            .execute("DELETE FROM machines WHERE machine_id=$1", &[&machine_id])
            .await
            .map_err(|err| {
                DbClientError(format!("failed to delete machine={machine_id}. {err}"))
            })?;
        Ok(())
    }

    pub async fn get_all_machines(&self) -> Result<Vec<Machine>, DbClientError> {

        let rows = self
            .client
            .query("SELECT * FROM machines", &[])
            .await
            .map_err(|err| DbClientError(format!("failed to retrieve all machines. {err}")))?;

        let mut results = vec![];

        for r in rows {
            let m = Machine::from_row(r)
                .map_err(|err| DbClientError(format!("get all machines failed. {:?}", err)))?;
            results.push(m)
        }
        Ok(results)
    }

    pub async fn get_machine_by(
        &self, 
        filters: Vec<(Queryable, &(dyn ToSql + Sync))>
    ) -> Result<Machine, DbClientError>{
        let mut base_query = String::from("SELECT * FROM machines WHERE ");

        for (ix, f) in filters.iter().enumerate(){
            let clause = format!("{}=${} ", f.0, ix+1);
            base_query.push_str(clause.as_str());
        };
        let params: Vec<&(dyn ToSql + Sync)> = filters.into_iter()
            .map(|f | f.1 as _).collect();
        
        let row = self
            .client
            .query_one(&base_query, &params)
            .await
            .map_err(|err| DbClientError(format!("failed to retrieve machine. {err}")))?;

        Ok(Machine::from_row(row)
                .map_err(|err| DbClientError(format!("unable to parse machine from row. {:?}", err)))?)
    }
    pub async fn get_machine_by_deployment(){}
}
