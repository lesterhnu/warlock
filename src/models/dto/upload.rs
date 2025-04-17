use time::PrimitiveDateTime;

use crate::entity::uploads;

pub struct UploadFileInfo {
    pub file_name: String,
    pub file_ext_name: String,
    pub file_source_name: String,
    pub create_time: PrimitiveDateTime,
}
impl From<uploads::Model> for UploadFileInfo {
    fn from(model: uploads::Model) -> Self {
        UploadFileInfo {
            file_name: model.file_name,
            file_ext_name: model.file_ext_name,
            file_source_name: model.file_source_name,
            create_time: model.create_time.into(),
        }
    }
}