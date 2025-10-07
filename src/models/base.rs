use serde::{Deserialize, Serialize};




#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BaseResponse<T> {
    pub code: i32,
    pub data: Option<T>,
    pub error_msg: Option<String>,
}