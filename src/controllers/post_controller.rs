use rocket::{
    Route,routes,
    get,post,put,delete,
    serde::json::Json,
    http::Status,
};
use serde::{
    Serialize,
};
use crate::{
    models::{
        Post,
    },
    schema::{
        posts,
    }
};


#[get("/?<page>")]
fn posts_show(page: Option<i32>) -> Status {
    let items_in_page = 10i32;
    Status::InternalServerError
}

pub fn routes() -> Vec<Route> {
    routes![posts_show]
}