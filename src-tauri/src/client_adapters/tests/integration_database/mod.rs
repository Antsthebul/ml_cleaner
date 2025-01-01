use deadpool_postgres::Object;
use postgres::NoTls;
use tokio_postgres::Client;

use crate::client_adapters::database::{create_connection_pool, DbClientError, build_test_conn_args};


mod project_db_int_test;
mod machine_db_int_test;
pub struct MockDbClient{}

impl MockDbClient{
    pub async fn new() -> Result<Object, crate::client_adapters::database::DbClientError> {
        let pool = create_connection_pool(build_test_conn_args());
        let c = pool.get().await.unwrap();
        
        Ok(c)
    }
}
