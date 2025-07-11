use serde::Deserialize;

pub mod user;
pub mod upload;
pub mod book;



#[derive(Debug,Deserialize)]
pub struct CommonReq {
    #[serde(default = "default_page")]
    pub page: Option<u64>,
    #[serde(default = "default_page_size")]
    pub page_size: Option<u64>,
}

fn default_page() -> Option<u64> {
    Some(0)
}
fn default_page_size() -> Option<u64> {
    Some(10)
}