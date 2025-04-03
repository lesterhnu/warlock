use crate::error::MyError;

pub mod base;
pub mod user;

pub async fn not_found() -> crate::Result<()> {
   Err( MyError::from_code_msg(404, "not found".to_string()))
}