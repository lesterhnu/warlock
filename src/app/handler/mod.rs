pub mod base;
pub mod user;
pub mod banner;

pub async fn not_found() -> crate::Result<()> {
   tracing::error!("not found");
   Err( crate::error::MyError::from_code_msg(404, "not found".to_string()))
   
}