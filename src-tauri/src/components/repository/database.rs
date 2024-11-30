use core::fmt;

use tokio_postgres::{NoTls, Error, Client};
#[derive(Debug)]
pub struct DbClientError(String);
#[derive(Debug)]
pub struct DbClient{}

impl DbClient{
    pub async fn new() -> Result<Client, DbClientError>{
        let (c, conn) = tokio_postgres::connect("host=host.docker.internal user=ml_cleaner password=ml_cleaner dbname=local_db", NoTls).await
        .map_err(|err| DbClientError(err.to_string()))?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Connection err: {}", e)
            }
        });
        Ok(c)
    }
}
impl fmt::Display for DbClientError{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        write!(f, "{}", self.0)
    }
}