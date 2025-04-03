
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::Result;
pub fn bcrypt_sth(s: String) -> Result<String> {
    Ok(hash(s, DEFAULT_COST)?)
}

pub fn bcrypt_verify(password:&str,hashed: &str) -> Result<bool> {
    Ok(verify(password, hashed)?)
}