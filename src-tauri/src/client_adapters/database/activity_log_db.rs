use std::{fmt::Display, str::FromStr};

use deadpool_postgres::Object;

use super::DbClientError;

#[derive(Debug)]
pub enum ActivityLogAction{
    Create,
    Update,
    Delete,
    Start,
    Stop,
    TrainStart,
    TrainStop
}

impl FromStr for ActivityLogAction{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "create" => Ok(Self::Create),
            "update" => Ok(Self::Update),
            "delete"=> Ok(Self::Delete),
            "start" => Ok(Self::Start),
            "Stop" => Ok(Self::Stop),
            "trainstart" => Ok(Self::TrainStart),
            "trainstop" => Ok(Self::TrainStop),
            _ => Err(())
        }
    }
}
impl Display for ActivityLogAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{:?}", self)
    }
}

pub struct ActivityLogDb{
    pub client: Object
}

impl ActivityLogDb{
    pub async fn create_activity_log(&self, action:ActivityLogAction, text:&str) -> Result<(), DbClientError>{
        let data = format!("{action}. {text}");

        let _ = self.client.execute("INSERT INTO activity_logs action (text) VALUES ($1)", &[&data])
            .await.map_err(|err|DbClientError(format!("failed to create activitiy_log. {err}")));
        Ok(())
    }
}