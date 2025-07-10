use axum::response::IntoResponse;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum MyError {
    #[error("error info: {1}")]
    WithCodeMsg(i32, String),
    #[error("An error occurred")]
    Default,

    #[error("An error occurred: {0}")]
    ConfigError(#[from] config::ConfigError),
    #[error("err-info:{0}")]
    RedisError(#[from] redis::RedisError),
    #[error("err-info:{0}")]
    DbError(#[from] sea_orm::error::DbErr),
    #[error("err-info:{0}")]
    CryptoError(#[from] bcrypt::BcryptError),
    #[error("err-info:{0}")]
    MultipartError(#[from] axum::extract::multipart::MultipartError),
    #[error("invalid parameter: {0}")]
    ParamError(#[from] validator::ValidationErrors),
    #[error("request error: {0}")]
    RequestError(#[from] reqwest::Error),
    #[error("template error: {0}")]
    TemplateError(#[from] askama::Error),
}

impl MyError {
    pub fn from_code_msg(code: i32, msg: String) -> Self {
        Self::WithCodeMsg(code, msg)
    }
    pub fn from_msg(msg: String) -> Self {
        Self::from_code_msg(-1, msg)
    }
}

#[derive(Debug, Serialize)]
struct ErrResp {
    code: i32,
    msg: String,
}

impl IntoResponse for MyError {
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            MyError::Default => ErrResp {
                code: -1,
                msg: "内部错误".to_string(),
            },
            MyError::WithCodeMsg(code, msg) => ErrResp { code, msg },
            _ => ErrResp {
                code: -1,
                msg: self.to_string(),
            },
        };
        if resp.code == 404 {
            return (axum::http::StatusCode::NOT_FOUND, axum::Json(resp)).into_response();
        }
        (axum::http::StatusCode::OK, axum::Json(resp)).into_response()
    }
}

/// --- Error Handling ---

#[derive(Debug)]
pub struct AppError(anyhow::Error);

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let err = self.0;
        let resp = ErrResp {
            code: -1,
            msg: err.to_string(),
        };
        (axum::http::StatusCode::OK, axum::Json(resp)).into_response()
    }
}

// 自动将 anyhow::Error 转换为 AppError
impl<E> From<E> for AppError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
