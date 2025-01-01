use std::net::Ipv4Addr;

use tokio_postgres::Client;

use crate::client_adapters::{
    model_hub::MachineState, 
    utils::ParseError
};

use super::DbClientError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct MachineCreate{
    pub machine_id: String,
    pub model: String,
    pub price:f32
}

#[derive(Deserialize)]
pub struct Machine{
    pub machine_id: String,
    pub model: String,
    pub price:f32,
    pub state:MachineState,
    pub ip_address: Option<Ipv4Addr>
}

impl Machine {
    pub fn from_ser_map(data_str:&str)-> Result<Machine, ParseError>{
        Ok(serde_json::from_str::<Machine>(data_str)
            .map_err(|err|ParseError(format!("unable to parse from_str. {err}")))?)
    }
}

impl MachineCreate {
    /// `data_str` MUST be in the form of serialized mapping
    pub fn from_ser_map(data_str:&str)-> Result<MachineCreate, ParseError>{
        Ok(serde_json::from_str::<MachineCreate>(data_str)
            .map_err(|err|ParseError(format!("unable to parse from_str. {err}")))?)
    }
}

pub struct MachineDb{
    pub client: Client
}

impl MachineDb{
    pub async fn create_machine(&self, data:MachineCreate ) -> Result<(), DbClientError>{
        let _ = self.client.execute("INSERT INTO machines 
            (machine_id, model, price, state) VALUES ($1,$2,$3, $4)",
            &[&data.machine_id, &data.model, &data.price, &MachineState::Off])
                .await.map_err(|err| DbClientError(format!("unable to create machine {err}")))?;
            
         
            Ok(())
        }
    
    pub async fn get_machine_by_id(&self, machine_id:&str) -> Result<Machine, DbClientError>{
        let rows = self.client.query("SELECT * FROM machines WHERE machine_id=$1", &[&machine_id])
            .await.map_err(|err|DbClientError(format!("unable to retrieve machine using machine_id={machine_id}, {err}")))?;

        if rows.len() == 0{
            return Err(DbClientError(format!("machine={machine_id} not found")))
        }else if rows.len() > 1{
            return Err(DbClientError(format!("multiples rows found for machine={machine_id}")))
        }
        let row = &rows[0];
        let mut ip_address = None;
        if let Some(ip_addr) = row.get::<_, Option<String>>("ip_address"){
            ip_address = ip_addr.parse().ok();
            if None  == ip_address {
                return Err(DbClientError("ip_address could not be parse".into()))
            }
            
        };
        Ok(Machine{
            machine_id: row.get("machine_id"),
            model: row.get("model"),
            price:row.get("price"),
            state:row.get("state"),
            ip_address:ip_address
        })

    }
    /// Operates like a 'PUT' action to update `machines`
    pub async fn update_machine(&self, data: Machine) -> Result<(), DbClientError>{
        let _ = self.client.execute("UPDATE machines 
            SET model=$2, price=$3, state=$4
            WHERE machine_id=$1",
            &[&data.machine_id, &data.model, &data.price, &data.state])
                .await.map_err(|err| DbClientError(format!("unable to create machine {err}")))?;
            
         
            Ok(())
        }

    pub async fn delete_machine(&self, machine_id:&str) -> Result<(), DbClientError>{
        let _ = self.client.execute("DELETE FROM machines WHERE machine_id=$1", &[&machine_id])
            .await.map_err(|err| DbClientError(format!("failed to delete machine={machine_id}. {err}")));
        Ok(())
    }
}