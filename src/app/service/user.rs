use crate::dto::user::{CreateUserReq,LoginReq, User};
use crate::pkg::utils;
use crate::resp::AppResp;
use crate::{crypto, entity, get_db, MyError, Result};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, QueryFilter};
use sea_orm::Set;

// 创建用户
pub async fn create_user(_user: CreateUserReq) -> Result<AppResp<()>> {
    let conn = get_db()?;
    let crypto_password = crypto::bcrypt_sth(_user.password.clone())?;
    let am = entity::users::ActiveModel {
        id: Default::default(),
        username: Set(_user.username),
        password: Set(crypto_password),
        email: Set(Some(_user.email)),
        ..Default::default()
    };
    am.insert(conn).await?;
    Ok(AppResp::Success)
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

// login
pub async fn login(user: LoginReq) -> Result<AppResp<User>> {
    let conn = get_db()?;
    let record = entity::users::Entity::find()
        .filter(entity::users::Column::Username.eq(user.username))
        .one(conn)
        .await?;
    match record{
        Some(m) => {
            tracing::info!("user found: {:?}", m);
            if !crypto::bcrypt_verify(&user.password,m.password.as_str() )? {
                return Err(MyError::from_msg("密码错误".to_string()));
            }
            //更新登录时间
            let mut am = m.clone().into_active_model();
            am.last_login = Set(Some(utils::get_local_time()));
            am.update(conn).await?;
            Ok(AppResp::SuccessWithData(User::from(m)))
        },
        None => {
            return Err(MyError::from_msg("用户不存在".to_string()));
        }
    }
}