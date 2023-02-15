use rocket::{
    get,post,
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
        User,
        AccessToken,
    },
    schema::{
        users,
        access_tokens,
    },
};
use regex::Regex;

#[derive(Serialize)]
enum SignupResponse{
    user{
        id: i32,
        username: String,
    },
    errors{
        username: bool,
        password: bool,
        unexpected: bool,
    },
}
#[derive(Deserialize,Insertable)]
#[diesel(table_name = users)]
struct NewUser{
    username:String,
    password:String,
}
#[post("/", data = "<inputs>")]
fn signup(inputs: Json<NewUser>) -> Json<GenericResponse<SignupResponse>> {
    let username = inputs.username.trim().to_lowercase();
    let flag_username = {
        !validate_username(&username)
    };
    let flag_password = {
        !validate_password(inputs.password.trim())
    };
    if flag_username || flag_password {
        return Json(GenericResponse{
            code:422,
            status:"Unprocessable Entity".to_string(),
            response: SignupResponse::errors{username: flag_username, password: flag_password, unexpected: false},
        });
    }
    let connection = &mut crate::establish_connection();
    let result = diesel::
        insert_into(crate::schema::users::table)
        .values(NewUser{
            username: inputs.username.to_string(),
            password: sha256::digest(&*inputs.password),
        })
        .get_result::<User>(connection);
    if result.is_ok() {
        let user = result.unwrap();
        return Json(GenericResponse{
            code: 201,
            status: "Created".to_string(),
            response: SignupResponse::user{id:user.id, username:user.username},
        });
    }
    Json(GenericResponse{
        code:409,
        status:"Conflict".to_string(),
        response: SignupResponse::errors{username: false, password: false, unexpected: true},
    })
}

fn validate_username(string: &str) -> bool {
    let re = Regex::new(r"^\w{4,30}$");
    re.unwrap().is_match(string)
}

fn validate_password(string: &str) -> bool {
    let rule1 = Regex::new(r"^[!$#@%a-zA-Z0-9]{8,40}$").unwrap();
    let rule2 = Regex::new(r"[a-zA-Z]").unwrap();
    let rule3 = Regex::new(r"[0-9]").unwrap();
    rule1.is_match(string) && rule2.is_match(string) && rule3.is_match(string)
}

use crate::guards::autheduser::AuthedUser;

#[get("/")]
fn whoami(user: AuthedUser) -> Json<AuthedUser> {
    Json(user)
}

pub fn routes() -> Vec<Route> {
    routes![signup, whoami]
}