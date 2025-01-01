pub mod project_db;
pub mod machine_db;
pub mod activity_log_db;

use core::fmt;

use tokio_postgres::{Client, NoTls};

#[cfg(test)]
mod machine_db_test;


#[derive(Debug)]
pub struct DbClientError(pub String);


pub trait AsyncDbClient {
    fn new() ->  impl std::future::Future<Output = Result<Client, DbClientError>>;
}

impl fmt::Display for DbClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct PGClient{
    client: Client
}

impl AsyncDbClient for PGClient {
    async fn new() -> Result<Client, DbClientError> {
        let (c, conn) = tokio_postgres::connect(
            "host=host.docker.internal user=ml_cleaner password=ml_cleaner dbname=local_db",
            NoTls,
        )
        .await
        .map_err(|err| DbClientError(err.to_string()))?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                eprintln!("Connection err: {}", e)
            }
        });
        Ok(c)
    }
}