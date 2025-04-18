use axum::extract::{Path,Json};
use crate::resp::AppResp;
use crate::{dto, service, Result};
use crate::dto::user::User;
use validator::Validate;

pub async fn get_user_by_id(Path(id): Path<i64>) -> Result<AppResp<User>> {
    let data = service::user::get_user_by_id(id).await?;
    Ok(AppResp::SuccessWithData(data))
}

pub async fn create_user(Json(user_info):Json<dto::user::CreateUserReq>) -> Result<AppResp<()>> {
    user_info.validate()?;
    crate::app::service::user::create_user(user_info).await?;
    Ok(AppResp::Success)
}

pub async fn login(Json(login_info):Json<dto::user::LoginReq>)->Result<AppResp<dto::user::User>>{
    login_info.validate()?;
    let data = service::user::login(login_info).await?;
    Ok(data)
}