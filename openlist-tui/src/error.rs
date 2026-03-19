use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("网络错误：{0}")]
    Network(#[from] reqwest::Error),

    #[error("认证失败：{0}")]
    Auth(String),

    #[error("路径不存在：{0}")]
    NotFound(String),

    #[error("API 错误：{0}")]
    ApiError(String),

    #[error("IO 错误：{0}")]
    Io(#[from] std::io::Error),

    #[error("配置错误：{0}")]
    Config(String),

    #[error("剧集解析失败：{0}")]
    EpisodeParse(String),

    #[error("正则表达式错误：{0}")]
    Regex(#[from] regex::Error),

    #[error("验证错误：{0}")]
    Validation(String),
}

pub type Result<T> = std::result::Result<T, AppError>;
