use deadpool_postgres::Object;

use super::DbClientError;


pub struct Tag{
    pub name: String,
    pub object_id: i32,
    pub model: String
}

pub struct TagDb{
    pub client: Object
}

impl TagDb{
    pub async fn create_tag(&self, 
        data: Tag) -> Result<(), DbClientError>{
            let _ = self.client
                .execute("INSERT INTO tags (name, model, object_id) VALUES ($1,$2,$3)", &[&data.name, &data.model, &data.object_id])
                .await
                .map_err(|err|DbClientError(format!("unable to create tag. {err}")))?;

            Ok(())
        }
    
    pub async fn delete_tag(&self, 
        tag: &Tag) -> Result<(), DbClientError>{
        let _ = self.client
            .execute("DELETE FROM tags WHERE object_id=$1 AND model=$2",
                 &[&tag.object_id, &tag.model])
            .await
            .map_err(|err|DbClientError(format!("unable to delete tag. {err}")))?;
        Ok(())
        }
    
    pub async fn get_tags_for_object(&self, model:&str, object_id:&i32)->Result<Vec<Tag>,DbClientError>{
        let rows = self.client
                .query("SELECT * FROM tags WHERE  object_id=$1 AND model=$2", &[&object_id, &model])
                .await
                .map_err(|err|DbClientError(format!("unable to get tags for model_={object_id} object_id={model}. {err}")))?;

        let mut tags:Vec<_> = vec![]; 
        for r in rows{
            tags.push(Tag{
                name: r.get("name"),
                object_id:r.get("object_id"),
                model:r.get("model")
            })
        }

        Ok(tags)
    }
}


