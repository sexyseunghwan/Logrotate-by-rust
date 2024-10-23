use crate::common::*;

pub type SettingsList = Vec<Settings>;

#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct Settings {
    pub component: String,
    pub log_path: String,
    pub file_name_pattern: String,
    pub log_retention_period: i32,
    pub compression_yn: bool,
    pub time_zone: String
}