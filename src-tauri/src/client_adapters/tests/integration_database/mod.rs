use deadpool_postgres::Object;
use postgres::NoTls;
use tokio_postgres::Client;

use crate::client_adapters::database::{
    build_test_conn_args, create_connection_pool, DbClientError,
};

mod machine_db_int_test;
mod machine_event_db_int;
mod project_db_int_test;

pub struct MockDbClient {}

impl MockDbClient {
    pub async fn new() -> Result<Object, crate::client_adapters::database::DbClientError> {
        let pool = create_connection_pool(build_test_conn_args());
        let c = pool.get().await.unwrap();

        Ok(c)
    }
}
