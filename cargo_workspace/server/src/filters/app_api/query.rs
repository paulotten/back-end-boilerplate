use super::context::Context;
use juniper::{graphql_object, GraphQLObject};

pub struct Query;

#[derive(GraphQLObject)]
struct PhotoSummary {
    photo_id: String,
    created_at: String,
}

#[derive(GraphQLObject)]
struct Photo {
    photo_id: String,
    created_at: String,
    photo_data: String,
}

#[graphql_object(context = Context)]
impl Query {
    async fn photos_list(context: &Context) -> Vec<PhotoSummary> {
        let mut pool = context.0.conn().await.unwrap();

        // skip photo_data when listing all photos
        sqlx::query_as!(
            PhotoSummary,
            "select photo_id::text as \"photo_id!\", created_at::text as \"created_at!\" from photos order by created_at desc",
        )
        .fetch_all(&mut pool)
        .await
        .unwrap()
    }

    async fn photos(context: &Context, photo_ids: Vec<String>) -> Vec<Photo> {
        let mut pool = context.0.conn().await.unwrap();
        let mut photos = vec![];
        let mut uuids = vec![];

        for id in photo_ids {
            if let Ok(uuid) = uuid::Uuid::parse_str(&id) {
                uuids.push(uuid);
            }
        }

        let rows = sqlx::query!(
            "select photo_id, created_at, photo_data from photos where photo_id = any($1) order by created_at desc",
            &uuids[..],
        )
        .fetch_all(&mut pool)
        .await
        .unwrap();

        for row in rows {
            let photo = Photo {
                photo_id: row.photo_id.to_string(),
                created_at: row.created_at.to_string(),
                photo_data: base64::encode(row.photo_data),
            };

            photos.push(photo);
        }

        photos
    }
}
