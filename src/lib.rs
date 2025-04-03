pub mod app;
pub mod boot;
pub mod error;
pub mod resp;
pub mod router;
pub mod pkg;



pub use pkg::crypto;
pub use boot::conf::CONFIG;


pub type Result<T> = std::result::Result<T, error::MyError>;