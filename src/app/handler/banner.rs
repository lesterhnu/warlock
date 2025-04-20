use axum::Json;

use crate::{Result,dto,AppResp,service};
pub async fn get_banner()->Result<AppResp<Vec<dto::banner::BannerDto>>>{
    let res = service::banner::get_banner().await?;
    Ok(AppResp::SuccessWithData(res))
}


pub async fn create_banner(Json(banner_list):Json<Vec<dto::banner::BannerDto>>)->Result<AppResp<()>>{
    service::banner::create_banner(banner_list).await?;
    Ok(AppResp::Success)
}