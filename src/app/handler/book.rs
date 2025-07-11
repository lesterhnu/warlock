use axum::Json;
use axum::extract::Query;

use crate::{AppResp, Result, dto, service};
pub async fn get_book(
    Query(req): Query<dto::CommonReq>,
) -> Result<AppResp<Vec<dto::book::Book>>> {
    let res = service::book::get_books(req).await?;
    Ok(AppResp::SuccessWithData(res))
}


pub async fn create_book(Json(req): Json<dto::book::Book>) -> Result<AppResp<()>> {
    service::book::create_book(req).await?;
    // Ok(())
    Ok(AppResp::Success)
}

// pub async fn delete_banner(Query(id):Query<i32>)->Result<AppResp<()>>{
//     service::banner::delete_banner(id).await?;
//     Ok(AppResp::Success)
// }

// pub async fn batch_delete_banner(Json(banner_list):Json<Vec<i32>>)->Result<AppResp<()>>{
//     service::banner::batch_delete_banner(banner_list).await?;
//     Ok(AppResp::Success)
// }
