use crate::dto::user::User;
use crate::{get_db, MyError, Result, entity};
use sea_orm::EntityTrait;

// 创建用户
pub async fn create_user(_user: User) -> Result<User> {
    todo!()
}

// 获取用户信息
pub async fn get_user_by_id(id: i64) -> Result<User> {
    let conn = get_db()?;
    let user = entity::users::Entity::find_by_id(id as i32)
        .one(conn)
        .await?
        .ok_or(MyError::Default)?
        .into();
    Ok(user)
}

// 删除用户
pub async fn delete_user(id: i64) -> Result<bool> {
    let conn = get_db()?;
    let res = entity::users::Entity::delete_by_id(id as i32)
        .exec(conn)
        .await?;
    tracing::info!("delete user: {}", id);
    Ok(res.rows_affected > 0)
}