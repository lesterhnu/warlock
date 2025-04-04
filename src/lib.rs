pub mod app;
pub mod boot;
pub mod error;
pub mod resp;
pub mod router;
pub mod pkg;
pub mod models;


pub use app::handler;
pub use app::service;

pub use models::entity;
pub use models::dto;
pub use pkg::crypto;
pub use boot::conf::CONFIG;
pub use boot::db::DB;
pub use error::MyError;


pub type Result<T> = std::result::Result<T, MyError>;

