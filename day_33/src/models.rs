use super::schema::rustaceans;
use diesel::{ Queryable, Insertable };
use serde::{ Serialize, Deserialize };

#[derive(Serialize, Queryable, Deserialize)]
pub struct Rustacean {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Deserialize, Insertable)]
#[diesel(table_name = rustaceans )]
pub struct NewRustacean {
    pub name: String,
    pub email: String,
}