use rocket::{
    request::{
        Request,
        FromRequest,
        Outcome,
    },
};

#[derive(serde::Serialize)]
pub struct RecivedToken{
    pub string: String,
}
#[derive(Debug)]
pub enum RecivedTokenError{
    Unexpected,
}
#[rocket::async_trait]
impl <'r> FromRequest<'r> for RecivedToken {
    type Error = RecivedTokenError;
    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error>{
        Outcome::Success(Self{string:req.headers().get_one("Authorization").unwrap_or("").to_string()})
    }
}