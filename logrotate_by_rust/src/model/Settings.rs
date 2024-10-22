use crate::common::*;


#[derive(Serialize, Deserialize, Debug, Getters)]
#[getset(get = "pub")]
pub struct Settings {
    pub log_path: String,
    pub log_retention_period: i32,
    pub compression_yn: bool,
    pub time_zone: String
}