#![allow(unused_imports)]
#![allow(non_camel_case_types)]
#[macro_use] extern crate rocket;

use rocket::{
    get,
    http::Status,
    serde::json::Json,
};
use::diesel::{
    prelude::*,
    pg::PgConnection,
};
use dotenvy::dotenv;
use std::env;


#[derive(serde::Serialize)]
pub struct GenericResponse<T>{
    pub code: i64,
    pub status: String,
    pub response: T,
}

#[get("/healthcheck")]
fn healthcheck() -> Json<GenericResponse<String>>{
    Json(
        GenericResponse{
            code: 200,
            status: "OK".to_string(),
            response: "".to_string(),
        }
    )
}

fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::
        var("DATABASE_URL")
        .expect("No connection specified");
    
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub mod schema;
pub mod models;
pub mod controllers;

#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env::set_var("RUST_BACKTRACE",env::var("RUST_BACKTRACE").unwrap());
    rocket::build()
        .mount("/",routes![healthcheck])
        .mount("/users/",controllers::user_controller::routes())
        .mount("/tokens/",controllers::token_controller::routes())
}
