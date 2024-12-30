use super::environment_db::get_all_projects;

#[tokio::test]
async fn test_wack(){
    get_all_projects().await.unwrap();
}