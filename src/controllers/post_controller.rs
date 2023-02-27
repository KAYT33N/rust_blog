use rocket::{
    Route,routes,
    get,post,put,delete,
    serde::json::Json,
    http::Status,
};
use serde::{
    Serialize,
    Deserialize,
};
use crate::{
    GenericResponse,
    models::{
        Post,
    },
    schema::{
        posts,
    },
    guards::{
        autheduser::AuthedUser,
    }
};
use diesel::{
    prelude::*,
    sql_types::{
        Integer,
        Text,
        Timestamp,
        BigInt,
        Varchar
    },
};


#[derive(Serialize)]
enum PostsShowResponses{
    errors{
        unexpected: bool,
    },
    posts(Vec<Post>),
}
#[get("/?<page>&<len>")]
fn posts_show_by_date(page: Option<i64>, len: Option<i64>) -> Json<GenericResponse<PostsShowResponses>> {
    let mut page:i64 = page.unwrap_or(1);
    if page < 1 {
        page = 1;
    }
    let mut items_in_page = len.unwrap_or(10i64);
    if items_in_page > 10 || items_in_page < 1{
        items_in_page = 10;
    }
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
        items_in_page => (200, "Ok"),
        _ => (206, "Last Page")
    };
    Json(GenericResponse{
        code: status.0,
        status: String::from(status.1),
        response: PostsShowResponses::posts(results)
    })
}


#[derive(Serialize, QueryableByName)]
struct RespondedPost{
    #[diesel(sql_type = Integer)]
    id: i32,
    #[diesel(sql_type = Integer)]
    parent_id: i32,
    #[diesel(sql_type = Integer)]
    user_id: i32,
    #[diesel(sql_type = Varchar)]
    username: String,
    #[diesel(sql_type = Text)]
    body: String,
    #[diesel(sql_type = Timestamp)]
    created_at: chrono::NaiveDateTime,
    #[diesel(sql_type = BigInt)]
    replies: i64,
}
#[derive(Serialize)]
enum PostFetchResponses{
    errors{
        post_id: bool,
        unexpected: bool,
    },
    posts(Vec<RespondedPost>),
}
#[get("/<post_id>")]
fn posts_show_by_thread(post_id: i32) -> Json<GenericResponse<PostFetchResponses>> {
    let conn = &mut crate::establish_connection();
    let query = String::from(format!("select a.id, a.parent_id, b.id user_id , b.username , a.body, a.created_at, (with recursive comms as ( select id, parent_id from posts where id = a.id union all select c.id, c.parent_id from comms join posts c on c.parent_id = comms.id ) select count(id)-1 from comms  ) as replies from (posts a join users b on a.user_id = b.id) where a.parent_id = {0} OR a.id = {0}", post_id));
    let results = diesel::
        sql_query(&query)
        .get_results::<RespondedPost>(conn);
    if !results.is_ok() {
        return Json(GenericResponse{
            code: 500,
            status: String::from("Internal server err"),
            response: PostFetchResponses::errors{
                post_id: false,
                unexpected: true,
            }
        })
    }
    let results = results.unwrap() ;
    if results.len() == 0 {
        return Json(GenericResponse{
            code: 404,
            status: String::from("Not Found"),
            response: PostFetchResponses::errors{
                post_id: true,
                unexpected: false,
            }
        })
    }
    Json(GenericResponse{
        code: 200,
        status: String::from("Ok"),
        response: PostFetchResponses::posts(results)
    })
}



#[derive(Serialize)]
enum PostsCreateResponses{
    errors{
        parent_id: bool,
        authed: bool,
        body: bool,
        unexpected: bool,
    },
    post(Post),
}
#[derive(Deserialize, Insertable)]
#[diesel(table_name = posts)]
struct NewPost{
    parent_id: Option<i32>,
    user_id: Option<i32>,
    body: String,
}
#[post("/", data = "<inputs>")]
fn posts_store(inputs: Json<NewPost>, user: AuthedUser) -> Json<GenericResponse<PostsCreateResponses>>{
    if inputs.body.len() < 3 {
        return Json(GenericResponse{
            code: 422,
            status: String::from("Unprocessable Entity"),
            response : PostsCreateResponses::errors{
                parent_id: false,
                authed: false,
                body: true,
                unexpected: false,
            },
        })
    }
    let parent_id = inputs.parent_id.unwrap_or(0);
    if parent_id != 0 {
        let conn = &mut crate::establish_connection();
        let flag = !posts::table
            .find(parent_id)
            .get_result::<Post>(conn)
            .is_ok();
        if flag {
            return Json(GenericResponse{
                code: 422,
                status: String::from("Unprocessable Entity"),
                response : PostsCreateResponses::errors{
                    parent_id: true,
                    authed: false,
                    body: false,
                    unexpected: false,
                },
            })
        }
    }
    let conn = &mut crate::establish_connection();
    let result = diesel::
        insert_into(posts::table)
        .values(NewPost{
            parent_id: Some(parent_id),
            user_id: Some(user.user_id),
            body: String::from(inputs.body.trim()),
        })
        .get_result::<Post>(conn);
    if !result.is_ok() {
        return Json(GenericResponse{
            code: 500,
            status: String::from("Internal Server Error"),
            response : PostsCreateResponses::errors{
                parent_id: false,
                authed: false,
                body: false,
                unexpected: true,
            },
        })
    }
    return Json(GenericResponse{
        code: 201,
        status: String::from("Created"),
        response : PostsCreateResponses::post(result.unwrap()),
    })
}

pub fn routes() -> Vec<Route> {
    routes![
        posts_show_by_date, 
        posts_store, 
        posts_show_by_thread
        ]
}