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
    GenericResponse,
    models::{
        Post,
    },
    schema::{
        posts,
    }
};
use diesel::prelude::*;


#[derive(Serialize)]
enum PostsShowResponses{
    errors{
        unexpected: bool,
    },
    posts(Vec<Post>),
}
#[get("/?<page>")]
fn posts_show(page: Option<i64>) -> Json<GenericResponse<PostsShowResponses>> {
    let mut page:i64 = page.unwrap_or(1);
    if page < 1 {
        page = 1;
    }
    let items_in_page = 10i64;
    let conn = &mut crate::establish_connection();
    let results = posts::table
        .offset(items_in_page*(page-1))
        .limit(items_in_page)
        .get_results(conn);
    if !results.is_ok(){
        return Json(GenericResponse{
            code: 500,
            status: String::from("Internal Server Error"),
            response: PostsShowResponses::errors{unexpected:true}
        });
    }
    let results = results.unwrap();
    let status = match results.len() {
        0 => (204, "No Content"),
        10 => (200, "Ok"),
        _ => (206, "Last Page")
    };
    Json(GenericResponse{
        code: status.0,
        status: String::from(status.1),
        response: PostsShowResponses::posts(results)
    })
}

pub fn routes() -> Vec<Route> {
    routes![posts_show]
}