use crate::{dto::upload::UploadFileInfo, error::MyError, pkg::utils, resp::AppResp, Result};
use axum::{extract::Multipart, response::Html};
use tokio::fs::File;
use askama::Template;

pub async fn ping() -> String{
    "ping".to_string()
    // let s = crate::pkg::ali::oss::get_sign().unwrap();
    // s
}
pub async fn test_config_error()->Result<()>{
    Err(MyError::ConfigError(config::ConfigError::NotFound("test".to_string())).into())
}

pub async fn get_oss_sign()->Result<AppResp<crate::pkg::ali::oss::UploadSignInfo>>{
    let res = crate::pkg::ali::oss::get_sign()?;
    Ok(AppResp::SuccessWithData(res))
}

pub async fn upload_file(mut multi:Multipart) -> Result<AppResp<()>> {
    while let Some(field) = multi.next_field().await? {
        let file_name = field.file_name().map(|name|name.to_string()).unwrap_or_else(||utils::new_uuid());
        let file_path = format!("storage/{}-{}",  utils::new_uuid(),file_name);
        File::create(&file_path).await.map_err(|err|{
            tracing::debug!("create file error:{}",err);
            MyError::from_msg(format!("create file error:{}",err))
        })?;
        let m = UploadFileInfo{
            file_name:file_path,
            file_ext_name:file_name.clone().split(".").last().unwrap_or("unknown").to_string(),
            // file_ext_name:field.content_type().map(|ct|ct.to_string()).unwrap_or_else(||"".to_string()),
            file_source_name:file_name,
            create_time:utils::get_local_time(),
        };
        crate::app::service::uploads::create_upload_info(m).await?;
    }
    tracing::info!("success");
    Ok(AppResp::Success)
}

pub async fn test_reqwest()->Result<Html<String>>{
    let resp = crate::ReqClient.get("https://www.baidu.com").send().await?;
    let res = resp.text().await?;

    Ok(Html(res))
}


#[derive(Debug,Template)]
#[template(path = "iframe_test.html")]
pub struct IframeTestTemplate;

pub async fn iframe_test_ui()->Result<Html<String>>{
    let t = IframeTestTemplate {};
    Ok(Html(t.render()?))
}