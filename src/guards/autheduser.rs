use rocket::{
    http::Status,
    request::{
        Request,
        FromRequest,
        Outcome,
    },
};
use serde::Serialize;
use crate::schema::access_tokens;
use crate::models::AccessToken;
use diesel::{
    prelude::*,
    sql_types::Bool,
    dsl::sql,
};

#[derive(Serialize)]
pub struct AuthedUser{
    pub id: i32,
}
#[derive(Debug)]
pub enum AuthError {
    TokenNotFound,
    InvalidToken,
    Unexpected,
}
#[rocket::async_trait]
impl <'r> FromRequest<'r> for AuthedUser{
    type Error = AuthError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<AuthedUser, AuthError>{
        let token_option = req.headers().get_one("Authorization");
        if token_option.is_none() {
            return Outcome::Failure((Status::Unauthorized, AuthError::TokenNotFound));
        }
        let connection = &mut crate::establish_connection();
        let token = token_option.unwrap();
        let result = crate::schema::access_tokens::table
            .filter(access_tokens::hashed.eq(sha256::digest(token.trim())))
            .filter(sql::<Bool>("NOW() < created_at + '1 hour'::interval * age"))
            .load::<AccessToken>(connection);
        if !result.is_ok(){
            return Outcome::Failure((Status::Unauthorized, AuthError::Unexpected));
        }
        let result_unwrapped = result.unwrap().pop();
        if result_unwrapped.is_none() {
            return Outcome::Failure((Status::Unauthorized, AuthError::InvalidToken));
        }
        Outcome::Success(AuthedUser{id: result_unwrapped.unwrap().user_id})
    }
}