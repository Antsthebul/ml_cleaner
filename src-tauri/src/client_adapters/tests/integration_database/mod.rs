use deadpool_postgres::Object;

use crate::client_adapters::database::{
    build_test_conn_args, create_connection_pool,
};

mod machine_db_int_test;
mod machine_event_db_int;
mod project_db_int_test;
mod deployment_db_int_test;

pub struct MockDbClient {}

impl MockDbClient {
    pub async fn new() -> Result<Object, crate::client_adapters::database::DbClientError> {
        let pool = create_connection_pool(build_test_conn_args());
        let c = pool.get().await.unwrap();

        Ok(c)
    }
}

async fn reset_database(client: &Object){

    for t in ["machine_events", "machines","deployments", "projects"]{

        let _ = client.execute(&format!("DELETE FROM {t}"), &[]).await.unwrap();
    }
        
}

pub async fn setup_database() -> Object{
    let client = MockDbClient::new().await.unwrap();
    reset_database(&client)
        .await;
    client
}