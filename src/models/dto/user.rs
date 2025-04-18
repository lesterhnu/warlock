use serde::{Deserialize, Serialize};
use crate::models::entity::users;
use validator::Validate;

#[derive(Debug,Deserialize,Validate,Serialize)]
pub struct CreateUserReq {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    #[validate(email(message="Invalid email format"))]
    pub email: String,
    #[validate(length(min = 6, max = 20))]
    pub password: String,
    #[validate(must_match(other="password",message="Passwords do not match"))]
    #[serde(rename = "confirm_password")]
    pub _confirm_password: String,
}

#[derive(Debug,Deserialize,Validate,Serialize)]
pub struct LoginReq {
    #[validate(length(min = 3, max = 20))]
    pub username: String,
    pub password: String,
}


#[derive(Debug,Serialize,Clone)]
pub struct User{
    pub id: i64,
    pub username: String,
    pub email:String,
    #[serde(skip_serializing_if="Option::is_none")]
    pub last_login:Option<String>,
}
impl From<users::Model> for User{
    fn from(u: users::Model) -> Self {
        let last_login = u.last_login.map(|t| t.to_string());
        User{
            id: u.id as i64,
            username: u.username,
            email: u.email.unwrap_or("".to_string()),
            last_login: last_login,
        }
    }
}