use super::context::Context;
use chrono::Utc;
use juniper::graphql_object;
use uuid::Uuid;

pub struct Mutation;

#[graphql_object(context = Context)]
impl Mutation {
    async fn save_photo(context: &Context, base64_data: String) -> bool {
        // parse base64_data
        let data = match base64::decode(base64_data) {
            Ok(d) => d,
            Err(_) => return false,
        };

        // generate uuid
        let uuid = Uuid::new_v4();

        // generate timestamp
        let now = Utc::now();
        let timestamp = now.timestamp_millis();

        // save to database
        let mut pool = context.0.conn().await.unwrap();
        sqlx::query!("insert into photos (photo_id, created_at, photo_data) values ($1, $2, $3)", uuid, timestamp, data).execute(&mut pool).await.unwrap();

        true
    }
}
