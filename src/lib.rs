pub mod app;
pub mod boot;
pub mod error;
pub mod resp;
pub mod router;
pub mod pkg;
pub mod models;


pub use app::handler;
pub use app::service;
pub use app::dao;


pub use models::entity;
pub use models::dto;
pub use pkg::crypto;
pub use boot::conf::CONFIG;
pub use boot::conf::CFG;
pub use boot::ReqClient;
pub use boot::db::get_db;
pub use error::{MyError,AppError};
pub use resp::AppResp;


// pub type Result<T> = std::result::Result<T,AppError>;

pub type Result<T> = std::result::Result<T, MyError>;

