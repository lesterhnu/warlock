use axum::extract::Path;
use crate::resp::AppResp;
use crate::{service, Result};
use crate::dto::user::User;
pub async fn get_user_by_id(Path(id): Path<i64>) -> Result<AppResp<User>> {
    let data = service::user::get_user_by_id(id).await?;
    Ok(AppResp::SuccessWithData(data))
}