use serde::Serialize;
use crate::models::entity::users;

#[derive(Debug,Serialize,Clone)]
pub struct User{
    pub id: i64,
    pub username: String,
    pub email:String,
    pub last_login:String,
}
impl From<users::Model> for User{
    fn from(u: users::Model) -> Self {
        User{
            id: u.id as i64,
            username: u.username,
            email: u.email.unwrap_or("".to_string()),
            last_login: u.last_login.to_string(),
        }
    }
}