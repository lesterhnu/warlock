pub mod admin;
pub mod base;
pub mod book;
pub mod user;

pub async fn not_found() -> crate::Result<()> {
   tracing::error!("not found");
   // Err(crate::AppError::from(anyhow::anyhow!("not found")))
   Err(crate::MyError::from_msg("not found".to_string()))
   
}