use crate::{AppResp, Result, dto, service};
pub async fn get_book() -> Result<AppResp<Vec<dto::book::Book>>> {
    let res = service::book::get_books().await?;
    Ok(AppResp::SuccessWithData(res))
}

// pub async fn create_banner(Json(banner_list):Json<Vec<dto::banner::BannerDto>>)->Result<AppResp<()>>{
//     service::banner::create_banner(banner_list).await?;
//     Ok(AppResp::Success)
// }

// pub async fn delete_banner(Query(id):Query<i32>)->Result<AppResp<()>>{
//     service::banner::delete_banner(id).await?;
//     Ok(AppResp::Success)
// }

// pub async fn batch_delete_banner(Json(banner_list):Json<Vec<i32>>)->Result<AppResp<()>>{
//     service::banner::batch_delete_banner(banner_list).await?;
//     Ok(AppResp::Success)
// }
