// use askama_axum::Template;
use askama::Template;
use axum::response::Html;
use crate::Result;

#[derive(Debug,Template)]
#[template(path = "admin/login.html")]
pub struct LoginTemplate;

pub async fn login_ui() ->Result<Html<String>>{
    let t = LoginTemplate {};
    Ok(Html(t.render()?))
}

#[derive(Debug,Template)]
#[template(path = "admin/upload.html")]
pub struct UploadTemplate;

pub async fn upload_ui() ->Result<Html<String>>{
    let t = UploadTemplate {};
    Ok(Html(t.render()?))
}