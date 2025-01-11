use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, Utc};
use deadpool_postgres::Object;
use postgres::Row;
use postgres_types::ToSql;
use toml::value::Datetime;

use super::DbClientError;


#[derive(Debug)]
pub enum MachineEventAction{
    Start,
    Stop,
    TrainStart,
    TrainStop
}

pub struct MachineEvent{

    action: MachineEventAction,
    created_at: DateTime<Utc>,
    machine_id: String
}

impl FromStr for MachineEventAction{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "start" => Ok(Self::Start),
            "Stop" => Ok(Self::Stop),
            "trainstart" => Ok(Self::TrainStart),
            "trainstop" => Ok(Self::TrainStop),
            _ => Err(())
        }
    }
}

impl Display for MachineEventAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{:?}", self)
    }
}

pub struct MachineEventDb{
    pub client: Object
}

impl MachineEventDb{
    
    pub async fn create_machine_event(&self, action: MachineEventAction, machine_id:&str) -> Result<(), DbClientError>{
        self.client.query("INSERT INTO machine_events (action, machine_id) VALUES ($1, $2)", 
        &[&action.to_string(), &machine_id])
            .await
            .map_err(|err| DbClientError(
                format!("unable to create machine event for machine_id={machine_id} . {err}",)))?;

        Ok(())
    }

    pub async fn get_all_machine_events(&self, machine_id:&str, 
        start_date:Option<DateTime<Utc>>, end_date:Option<DateTime<Utc>>) 
        -> Result<Vec<MachineEvent>, DbClientError>{
            let query_string = "SELECT * FROM machine_events WHERE machine_id=$1";

            let mut args: Vec<Box<dyn ToSql + Sync>> = vec![];
            let machine_id_str = machine_id.to_owned();
            args.push(Box::new(machine_id_str));

            if let Some(start) = start_date {
                args.push(Box::new(start))
            }

            if let Some(end) = end_date {
                args.push(Box::new(end))
            }
            // coerces references of Box objects 
            let refs:Vec<&(dyn ToSql + Sync)> = args.iter().map(|a|a.as_ref()).collect();

            let rows = self.client.query(query_string, &refs)
                .await
                .map_err(|err| DbClientError(
                    format!("unable to get machine events for machine_id={machine_id} . {err}",)))?;

            let mut results = vec![];

            for r in rows{
                let action:String =  r.get("action");
                results.push(MachineEvent{

                    action:action.parse().unwrap(),
                    machine_id:r.get::<_, String>("machine_id"),
                    created_at:r.get::<_, DateTime<Utc>>("created_at")
                })
            };

            Ok(results)
    }
}