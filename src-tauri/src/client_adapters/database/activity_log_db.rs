use std::{fmt::Display, str::FromStr};

use deadpool_postgres::Object;

use super::DbClientError;

pub enum ActivityLogAction{
    CREATE,
    UPDATE,
    DELETE
}

impl FromStr for ActivityLogAction{
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "create" => Ok(Self::CREATE),
            "update" => Ok(Self::UPDATE),
            "delete"=> Ok(Self::DELETE),
            _ => Err(())
        }
    }
}
impl Display for ActivityLogAction{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            Self::CREATE=>write!(f,"CREATE"),
            Self::UPDATE=>write!(f,"UPDATE"),
            Self::DELETE=>write!(f,"DELETE")
        }
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