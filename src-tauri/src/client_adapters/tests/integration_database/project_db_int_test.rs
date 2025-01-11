use super::MockDbClient;
use crate::client_adapters::database::project_db::ProjectDb;

#[tokio::test]
async fn test_get_all_projects() {
    // ARRAMGE
    let client = MockDbClient::new().await.unwrap();

    let _ = client
        .execute("DELETE FROM deployments", &[])
        .await
        .unwrap();
    let _ = client.execute("DELETE FROM projects", &[]).await.unwrap();

    let data = vec!["test1", "test2"];

    for v in &data {
        let row = client
            .query_one(
                "INSERT INTO projects (name) VALUES ($1) RETURNING id",
                &[&v],
            )
            .await
            .unwrap();
        let db_id: i32 = row.get(0);
        let _ = client
            .execute(
                "INSERT INTO deployments (name, project_id) VALUES ($1, $2)",
                &[&format!("dep-{}", v), &db_id],
            )
            .await
            .unwrap();
    }

    let project_db = ProjectDb { client };

    // ACT
    let results = project_db.get_all_projects().await.unwrap();

    // ASSERT
    assert_eq!(results.len(), data.len())
}

#[tokio::test]
async fn get_project_by_name() {
    // ARRANGE
    let client = MockDbClient::new().await.unwrap();
    let project_name = "test1";

    let _ = client
        .execute("DELETE FROM deployments", &[])
        .await
        .unwrap();
    let _ = client.execute("DELETE FROM projects", &[]).await.unwrap();

    let data = vec![project_name, "test2"];

    for v in &data {
        let row = client
            .query_one(
                "INSERT INTO projects (name) VALUES ($1) RETURNING id",
                &[&v],
            )
            .await
            .unwrap();
        let db_id: i32 = row.get(0);
        let _ = client
            .execute(
                "INSERT INTO deployments (name, project_id) VALUES ($1, $2)",
                &[&format!("dep-{}", v), &db_id],
            )
            .await
            .unwrap();
    }

    let project_db = ProjectDb { client };

    // ACT
    let project = project_db.get_project_by_name(&project_name).await.unwrap();

    // ASSERT
    assert_eq!(project.name, project_name)
}

#[tokio::test]
async fn get_project_deployment_by_name() {
    // ARRANGE
    let client = MockDbClient::new().await.unwrap();
    let project_name = "test1";

    let _ = client
        .execute("DELETE FROM deployments", &[])
        .await
        .unwrap();
    let _ = client.execute("DELETE FROM projects", &[]).await.unwrap();

    let data = vec![project_name, "test2"];

    for v in &data {
        let row = client
            .query_one(
                "INSERT INTO projects (name) VALUES ($1) RETURNING id",
                &[&v],
            )
            .await
            .unwrap();
        let db_id: i32 = row.get(0);
        let _ = client
            .execute(
                "INSERT INTO deployments (name, project_id) VALUES ($1, $2)",
                &[&format!("dep-{}", v), &db_id],
            )
            .await
            .unwrap();
    }

    let project_db = ProjectDb { client };

    // ACT
    let deployment = project_db
        .get_project_deployment_by_name(&project_name, "dep-test1")
        .await
        .unwrap();

    // ASSERT
    assert_eq!(deployment.name, "dep-test1")
}

#[tokio::test]
async fn create_project() {
    // ARRANGE
    let client = MockDbClient::new().await.unwrap();
    let project_name = "test1";

    let _ = client
        .execute("DELETE FROM deployments", &[])
        .await
        .unwrap();
    let _ = client.execute("DELETE FROM projects", &[]).await.unwrap();

    let project_db = ProjectDb { client };

    // ACT
    let _ = project_db.upsert_project(&project_name).await.unwrap();

    // ASSERT
    let project = project_db.get_project_by_name(&project_name).await.unwrap();

    assert_eq!(project.name, project_name)
}

#[tokio::test]
async fn create_deployment() {
    // ARRANGE
    let client = MockDbClient::new().await.unwrap();
    let project_name = "test1";
    let deployment_name = "dep-test1";

    let _ = client
        .execute("DELETE FROM deployments", &[])
        .await
        .unwrap();
    let _ = client.execute("DELETE FROM projects", &[]).await.unwrap();

    let _ = client
        .execute("INSERT INTO projects (name) VALUES($1)", &[&project_name])
        .await;

    let project_db = ProjectDb { client };

    // ACT
    let _ = project_db
        .upsert_deployment(&project_name, &deployment_name)
        .await
        .unwrap();

    // ASSERT
    let deployment = project_db
        .get_project_deployment_by_name(&project_name, &deployment_name)
        .await
        .unwrap();

    assert_eq!(deployment.name, deployment_name)
}

#[tokio::test]
async fn delete_deployment() {
    // ARRANGE
    let client = MockDbClient::new().await.unwrap();
    let project_name = "test1";
    let deployment_name = "dep-test1";

    let _ = client
        .execute("DELETE FROM deployments", &[])
        .await
        .unwrap();
    let _ = client.execute("DELETE FROM projects", &[]).await.unwrap();

    let data = vec![project_name, "test2"];

    for v in &data {
        let row = client
            .query_one(
                "INSERT INTO projects (name) VALUES ($1) RETURNING id",
                &[&v],
            )
            .await
            .unwrap();
        let db_id: i32 = row.get(0);
        let _ = client
            .execute(
                "INSERT INTO deployments (name, project_id) VALUES ($1, $2)",
                &[&format!("dep-{}", v), &db_id],
            )
            .await
            .unwrap();
    }

    // ACT

    let project_db = ProjectDb { client };

    let _ = project_db
        .delete_deployment(&project_name, deployment_name)
        .await
        .unwrap();

    let error = project_db
        .get_project_deployment_by_name(&project_name, &deployment_name)
        .await
        .unwrap_err();

    assert!(error.0.contains("No rows found"))
}
