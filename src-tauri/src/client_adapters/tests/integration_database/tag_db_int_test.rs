use crate::client_adapters::database::tag_db::{Tag, TagDb};

use super::setup_database;

#[tokio::test]
async fn test_create_retrieve_and_delete_tag_success(){
    // ARRANGE
    let client = setup_database().await;

    // lets tag a project (since its the most simple to create)
    let project_name = "test1";
    let test_tag = "Tag1";
    let test_model = "projects";

    let r = client
        .query_one("INSERT INTO projects (name) VALUES($1) RETURNING id", &[&project_name])
        .await
        .unwrap();
    
    let object_id: i32 = r.get("id");
    
    
    let tag1 = Tag{
        object_id: object_id,
        name: test_tag.into(),
        model: test_model.into()
    };

    let tag2 = Tag{
        object_id: 5,
        name: "unkown".into(),
        model: "different".into()
    };
    let tags:Vec<_> = vec![tag1, tag2];

    let tag_db = TagDb{ client };

    // ACT (and ASSERT) - CREATE
    for t in tags{

        tag_db.create_tag(t).await.unwrap();
    }

    // ACT (AND ASSERT) - RETRIEVE
    let db_tags = tag_db.get_tags_for_object(test_model, &object_id)
        .await
        .unwrap();

    assert_eq!(db_tags.len(), 1);


    let recent_tag = Tag{
        object_id: object_id,
        model:test_model.into(),
        name: test_tag.into()
    };

    // ACT AND ASSERT - DELETE
    tag_db.delete_tag(&recent_tag)
        .await
        .unwrap();
    
    let tags = tag_db.get_tags_for_object(&test_model, &object_id)
        .await
        .unwrap();
  
    assert_eq!(tags.len(), 0);
}
