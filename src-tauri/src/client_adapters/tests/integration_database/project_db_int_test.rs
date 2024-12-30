use crate::client_adapters::database::project_db::ProjectDb;
use super::MockDbClient;

#[tokio::test]
async fn test_get_all_projects(){
    // ARRAMGE
    let client = MockDbClient::new().await.unwrap();

    let _ = client.execute("DELETE FROM deployments", &[])
        .await.unwrap();
    let _ = client.execute("DELETE FROM projects", &[])
        .await.unwrap();

    let data = vec!["test1", "test2"];
    
    for v in &data{
        let row = client.query_one("INSERT INTO projects (name) VALUES ($1) RETURNING id", &[&v])
            .await.unwrap();
        let db_id : i32 = row.get(0);
        let _ = client.execute("INSERT INTO deployments (name, project_id) VALUES ($1, $2)", &[&format!("dep-{}", v), &db_id])
            .await.unwrap();
    }

    let project_db = ProjectDb{client};
    
    // ACT
    let results = project_db.get_all_projects().await.unwrap();
    
    // ASSERT
    assert_eq!(results.len(), data.len())
}