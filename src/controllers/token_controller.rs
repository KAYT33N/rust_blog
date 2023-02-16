use rocket::{
    http::Status,
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
use dotenvy::dotenv;
use std::env;
use sha256::digest;
use rand::{distributions::Alphanumeric, Rng};

const TOKENS_LEN:usize = 30;

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
    let raw_token = generate_token(TOKENS_LEN);
    let result = diesel::
        insert_into(access_tokens::table)
        .values(NewToken{hashed: digest(raw_token.clone()), user_id: user.id, age: tokens_age()})
        .execute(connection);
    if result.is_ok() {
        return Json(Token{token: raw_token});
    }
    Json(Token{token: "unexpected".to_string()})
}

fn tokens_age() -> i32 {
    dotenv().ok();
    env::var("ACCESS_TOKENS_AGE")
        .unwrap_or("1".to_string())
        .parse::<i32>()
        .unwrap_or(1i32)
}

fn generate_token(len: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}

use crate::guards::recivedtoken::RecivedToken;
#[delete("/")]
fn logout(token: RecivedToken) -> Status { 
    let token = token.string.trim();
    if token.len() != TOKENS_LEN {
        return Status::Unauthorized;
    }
    let conn = &mut crate::establish_connection();
    let nums_deleted = 
        diesel::delete(
            access_tokens::table
                .filter(access_tokens::hashed.eq(sha256::digest(token)))
        ).execute(conn);
    if !nums_deleted.is_ok() {
        return Status::InternalServerError;
    }
    if nums_deleted.unwrap() == 0 {
        return Status::Unauthorized;
    }
    Status::Accepted
}

pub fn routes() -> Vec<Route> {
    routes![login,logout]
}