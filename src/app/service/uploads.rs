use sea_orm::ActiveModelTrait;

use crate::dto::upload::UploadFileInfo;
use crate::entity::uploads;
use crate::resp::AppResp;
use crate::{Result, get_db};

pub async fn create_upload_info(file_info: UploadFileInfo) -> Result<AppResp<UploadFileInfo>> {
    let conn = get_db()?;
    let m = uploads::ActiveModel {
        file_name: sea_orm::ActiveValue::Set(file_info.file_name),
        file_ext_name: sea_orm::ActiveValue::Set(file_info.file_ext_name),
        file_source_name: sea_orm::ActiveValue::Set(file_info.file_source_name),
        create_time: sea_orm::ActiveValue::Set(file_info.create_time),
        ..Default::default()
    };
    let res = m.insert(conn).await?;
    let r = UploadFileInfo::from(res);
    Ok(AppResp::SuccessWithData(r))
}
