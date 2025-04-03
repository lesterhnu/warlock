use serde::Serialize;
use axum::response::IntoResponse;

pub enum AppResp<T> {
    Success,
    Failed,
    SuccessWithData(T),
    FailedWithMsg(i32,String),
}

#[derive(Serialize, Debug)]
pub struct RespStruct<T> {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl <T:Serialize> IntoResponse for AppResp<T>{
    fn into_response(self) -> axum::response::Response {
        let resp = match self {
            AppResp::Success => RespStruct {
                code: 0,
                message: "Success".to_string(),
                data: None,
            },
            AppResp::Failed => RespStruct {
                code: -1,
                message: "Failed".to_string(),
                data: None,
            },
            AppResp::SuccessWithData(data) => RespStruct {
                code: 0,
                message: "Success".to_string(),
                data: Some(data),
            },
            AppResp::FailedWithMsg(code,msg) => RespStruct {
                code: code,
                message: msg,
                data: None,
            },
        };
        (axum::http::StatusCode::OK, axum::Json(resp)).into_response()
    }
}