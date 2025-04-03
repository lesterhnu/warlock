use crate::dto::user::User;
use crate::{DB, MyError, Result, entity};
use sea_orm::EntityTrait;

// 获取用户信息
pub async fn get_user_by_id(id: i64) -> Result<User> {
    let conn = DB.get()?;
    let user = entity::users::Entity::find_by_id(id)
        .one(conn)
        .await?
        .ok_or(MyError::Default)?
        .into();
    Ok(user)
}

// 删除用户
pub async fn delete_user(id: i64) -> Result<bool> {
    let conn = DB.get()?;
    entity::users::
}