use rocket::{
    post,delete,
    Route,routes,
    serde::json::Json,
};
use serde::{
    Serialize,
    Deserialize,
};
use diesel::prelude::*;
use crate::{
    GenericResponse,
    models::{
        AccessToken,
        User,
    },
    schema::{
        users,
        access_tokens,
    },
};
use sha256::digest;
use rand::{distributions::Alphanumeric, Rng};

#[derive(Serialize)]
struct Token{token: String}
#[derive(Deserialize)]
struct LoginForm{
    username: String,
    password: String,
}
#[derive(Insertable)]
#[diesel(table_name = access_tokens)]
struct NewToken{
    hashed: String,
    user_id: i32,
    age: i32,
}
#[post("/", data = "<inputs>")]
fn login(inputs: Json<LoginForm>) -> Json<Token>{
    let connection = &mut crate::establish_connection();
    let result = users::table
        .filter(users::username.eq(inputs.username.trim()))
        .filter(users::password.eq(digest(inputs.password.trim())))
        .load::<User>(connection);
    if !result.is_ok() {
        return Json(Token{token: "user not found".to_string()});
    }
    let result_unwrapped = result.unwrap().pop();
    if result_unwrapped.is_none() {
        return Json(Token{token: "user not found".to_string()});
    }
    let user: User = result_unwrapped.unwrap();
    let raw_token = generate_token(30);
    let result = diesel::
        insert_into(access_tokens::table)
        .values(NewToken{hashed: digest(raw_token.clone()), user_id: user.id, age: 7})
        .execute(connection);
    if result.is_ok() {
        return Json(Token{token: raw_token});
    }
    Json(Token{token: "unexpected".to_string()})
}

fn generate_token(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

pub fn routes() -> Vec<Route> {
    routes![login]
}