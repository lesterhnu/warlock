use serde::{Deserialize, Serialize};

use crate::entity::banner::Model;



#[derive(Debug,Serialize,Default,Deserialize)]
pub struct BannerDto{
    pub id:i32,
    pub url:String,
    #[serde(skip_serializing,default="default_is_show")]
    pub is_show:i8,
    pub create_time:String,
}

impl From<Model> for BannerDto{
    fn from(model: Model) -> Self {
        BannerDto{
            id:model.id,
            url:model.url,
            create_time:model.create_time.to_string(),
            ..Default::default()
        }
    }
}
fn default_is_show()->i8{
    1
}