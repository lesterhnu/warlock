pub mod datetime;
pub use datetime::*;

pub fn new_uuid() -> String {
    uuid::Uuid::new_v4().to_string()
}