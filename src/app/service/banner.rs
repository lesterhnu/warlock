

use crate::{dto::banner::BannerDto, entity, pkg::utils, Result};
pub async fn get_banner()->Result<Vec<BannerDto>>{
    let res:Vec<BannerDto> = crate::dao::banner::get_banner_list()
    .await?.into_iter()
    .map(|item|BannerDto::from(item)).collect();

    Ok(res)
}

pub async fn create_banner(banner_list:Vec<BannerDto>)->Result<()>{
    let model_list = banner_list.into_iter().map(|item|entity::banner::Model{
        id:0,
        url:item.url,
        is_show:1,
        local_path:"".to_string(),
        create_time: utils::get_local_time(),
    }).collect();
    crate::dao::banner::create_banner(model_list).await?;
    Ok(())
}