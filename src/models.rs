use super::schema::likes;

use chrono::NaiveDateTime;

#[derive(Insertable, Queryable)]
#[table_name="likes"]
pub struct LikeModel {
    pub first: String,
    pub second: String,
    pub cookie: String,
    pub timestamp: NaiveDateTime    
}
