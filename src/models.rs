use diesel::prelude::*;
use serde::{Serialize, Deserialize};

use crate::schema::users;
#[derive(Serialize, Queryable, QueryableByName)]
#[diesel(table_name = users)]
pub struct User{
    pub id: i32,
    pub username: String,
    pub password: String,
}

use crate::schema::posts;
#[derive(Serialize, Queryable, QueryableByName)]
#[diesel(table_name = posts)]
pub struct Post{
    pub id: i32,
    pub user_id: i32,
    pub body: String,
    pub created_at: chrono::NaiveDateTime,
}

use crate::schema::access_tokens;
#[derive(Serialize, Queryable, QueryableByName)]
#[diesel(table_name = access_tokens)]
pub struct AccessToken{
    pub hashed: String,
    pub user_id: i32,
    pub age: i32,
    pub created_at: chrono::NaiveDateTime,
}