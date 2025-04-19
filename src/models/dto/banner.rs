use serde::Serialize;

use crate::entity::banner::Model;


#[derive(Debug,Serialize)]
pub struct BannerDto{
    pub id:i32,
    pub url:String,
    pub create_time:String,
}

impl From<Model> for BannerDto{
    fn from(model: Model) -> Self {
        BannerDto{
            id:model.id,
            url:model.url,
            create_time:model.create_time.to_string(),
        }
    }
}