use time::{OffsetDateTime, macros::format_description};

use crate::{Error, objects::ObjectDictionary};

#[derive(Clone, Debug, Default)]
pub struct FileInfo {
    pub file_name: String,
    pub file_version: String,
    pub file_revision: String,
    pub description: String,
    pub created_by: String,
}

impl FileInfo {
    pub fn to_eds_str(&self, _obj_dict: &ObjectDictionary) -> Result<String, Error> {
        let now = OffsetDateTime::now_local().unwrap_or_else(|_| OffsetDateTime::now_utc());
        let format_time = format_description!("[hour repr:12]:[minute][period case:upper]");
        let format_date = format_description!("[month padding:zero]-[day padding:zero]-[year]");
        let now_time = now.format(&format_time).unwrap();
        let now_date = now.format(&format_date).unwrap();

        let file_name = self.file_name.as_str();
        let file_version = self.file_version.as_str();
        let file_revision = self.file_revision.as_str();
        let description = self.description.as_str();
        let created_by = self.created_by.as_str();
        let modified_by = self.created_by.as_str();

        let file_info = format!(
            "\
[FileInfo]
FileName={file_name}
FileVersion={file_version}
FileRevision={file_revision}
EDSVersion=4.0
Description={description}
CreationTime={now_time}
CreationDate={now_date}
CreatedBy={created_by}
ModificationTime={now_time}
ModificationDate={now_date}
ModifiedBy={modified_by}

",
        );
        Ok(file_info)
    }
}
