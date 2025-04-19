use crate::{dto::banner::BannerDto, Result};
pub async fn get_banner()->Result<Vec<BannerDto>>{
    let res:Vec<BannerDto> = crate::dao::banner::get_banner_list()
    .await?.into_iter()
    .map(|item|BannerDto::from(item)).collect();

    Ok(res)
}