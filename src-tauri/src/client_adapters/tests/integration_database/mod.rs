use postgres::NoTls;
use tokio_postgres::Client;

use crate::client_adapters::database::DbClientError;


mod project_db_int_test;
mod machine_db_int_test;
pub struct MockDbClient{}

impl MockDbClient{
    pub async fn new() -> Result<tokio_postgres::Client, crate::client_adapters::database::DbClientError> {
        let (c, conn) = tokio_postgres::connect(
            "host=host.docker.internal user=ml_cleaner password=ml_cleaner dbname=test port=5433",
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
