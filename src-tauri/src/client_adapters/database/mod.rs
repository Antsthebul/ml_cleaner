pub mod activity_log_db;
pub mod machine_db;
pub mod machine_event_db;
pub mod project_db;
pub mod sq_builder;
pub mod deployment_db;
pub mod tag_db;

use core::fmt;

use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod, Runtime};
use tokio_postgres::{Client, NoTls};

#[cfg(test)]
mod machine_db_test;

#[derive(Debug)]
pub struct DbClientError(pub String);

impl Into<String> for DbClientError {
    fn into(self) -> String {
        self.0
    }
}
pub trait AsyncDbClient {
    fn new() -> impl std::future::Future<Output = Result<Client, DbClientError>>;
}

impl fmt::Display for DbClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct PGClient {
    client: Client,
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
pub struct ConnectionArgs {
    host: String,
    user: String,
    port: u16,
    password: String,
    name: String,
}
pub fn build_conn_args() -> ConnectionArgs {
    ConnectionArgs {
        host: "host.docker.internal".into(),
        user: "ml_cleaner".into(),
        port: 5432,
        password: "ml_cleaner".into(),
        name: "local_db".into(),
    }
}
pub fn build_test_conn_args() -> ConnectionArgs {
    ConnectionArgs {
        host: "host.docker.internal".into(),
        user: "ml_cleaner".into(),
        port: 5433,
        password: "ml_cleaner".into(),
        name: "test".into(),
    }
}

pub fn create_connection_pool(conn_args: ConnectionArgs) -> Pool {
    let mut cfg = Config::new();
    cfg.manager = Some(ManagerConfig {
        recycling_method: RecyclingMethod::Fast,
    });

    cfg.host = Some(conn_args.host);
    cfg.user = Some(conn_args.user);
    cfg.password = Some(conn_args.password);
    cfg.dbname = Some(conn_args.name);
    cfg.port = Some(conn_args.port);

    cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap()
}
