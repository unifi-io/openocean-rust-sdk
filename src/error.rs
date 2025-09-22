use thiserror::Error;


#[derive(Debug, Error)]
pub enum OpenoceanError {
    /// 网络相关错误（reqwest 层面的连接、超时等）
    #[error("network error: {0}")]
    Network(String),

    /// HTTP 非 2xx 状态
    #[error("http error: status={status}, content_type={content_type:?}, body={body}")]
    Http {
        status: u16,
        body: String,
        content_type: Option<String>,
    },

    /// JSON 解析错误
    #[error("parse error at {path}: {message}. body={body}")]
    Parse {
        message: String,
        path: String,
        body: String,
    },

    /// 其它 SDK 内部错误
    #[error("internal error: {0}")]
    Internal(String),
}

impl From<reqwest::Error> for OpenoceanError {
    fn from(err: reqwest::Error) -> Self {
        if err.is_timeout() {
            OpenoceanError::Network("timeout".into())
        } else {
            OpenoceanError::Network(err.to_string())
        }
    }
}

/*
impl From<serde_json::Error> for OpenoceanError {
    fn from(err: serde_json::Error) -> Self {
        OpenoceanError::Parse {
            message: err.to_string(),
            path: "".to_string(),
            body: "".to_string(),
        }
    }
}*/