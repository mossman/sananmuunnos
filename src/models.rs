use super::schema::likes;

use chrono::NaiveDateTime;
use rocket::serde::Serialize;

#[derive(Insertable, Queryable)]
#[table_name = "likes"]
pub struct LikeModel {
    pub first: String,
    pub second: String,
    pub cookie: String,
    pub timestamp: NaiveDateTime,
}

#[derive(Queryable, Serialize)]
pub struct LikesCount {
    pub first: String,
    pub second: String,
    pub count: i64,
}
