use crate::client_adapters::database::deployment_db::{DeploymentCreate, DeplyomentDb};

use super::setup_database;

#[tokio::test]
async fn test_deployment_is_created_and_retreived(){
    // ARRANGE
    let client = setup_database().await;
    let test_project = "TestProject1";
    let test_deployment_name = "TestDeploy1";

    let row = client
        .query_one(
            "INSERT INTO projects (name) VALUES ($1) RETURNING id",
            &[&test_project],
        )
        .await
        .unwrap();

    let data = DeploymentCreate{
        project_id:row.get(0),
        name: test_deployment_name.into(),
        tags:vec![],
        description:"".into()
    };

    let deployment_db = DeplyomentDb{client};

    deployment_db.create_deployment(data).await.unwrap();

    deployment_db.get_deployment_by_name(test_deployment_name)
        .await
        .unwrap();

    
}