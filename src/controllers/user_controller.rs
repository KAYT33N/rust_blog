use rocket::{
    get,post,
    Route,routes,
    serde::json::Json,
};
use serde::{
    Deserialize,
};
use diesel::prelude::*;
use crate::{
    GenericResponse,
    models::{
        User,
    },
    schema::{
        users,
        access_tokens,
    },
};

#[derive(Deserialize,Insertable)]
#[diesel(table_name = users)]
struct NewUser{
    username:String,
    password:String,
}
#[post("/", data = "<inputs>")]
fn signup(inputs: Json<NewUser>) -> Json<GenericResponse<User>> {
    let connection = &mut crate::establish_connection();
    let results = diesel::
        insert_into(crate::schema::users::table)
        .values(NewUser{
            username: inputs.username.to_string(),
            password: sha256::digest(&*inputs.password),
        })
        .get_results::<User>(connection)
        .unwrap();
    Json(GenericResponse{
        code: 201,
        status: "Created".to_string(),
        response: results.into_iter().nth(0).unwrap(),
    })
}

pub fn routes() -> Vec<Route> {
    routes![signup]
}