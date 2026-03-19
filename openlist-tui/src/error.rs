use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("网络错误：{0}")]
    Network(String),

    #[error("认证失败：{0}")]
    Auth(String),

    #[error("Token 已过期，请重新登录")]
    TokenExpired,

    #[error("路径不存在：{0}")]
    NotFound(String),

    #[error("API 错误：{0}")]
    ApiError(String),

    #[error("IO 错误：{0}")]
    Io(String),

    #[error("配置错误：{0}")]
    Config(String),

    #[error("剧集解析失败：{0}")]
    EpisodeParse(String),

    #[error("正则表达式错误：{0}")]
    Regex(String),

    #[error("验证错误：{0}")]
    Validation(String),
}

impl AppError {
    /// Check if this error is a 401 Unauthorized (token expired)
    pub fn is_unauthorized(&self) -> bool {
        matches!(self, AppError::TokenExpired)
    }

    /// Check if this error is a network error
    pub fn is_network_error(&self) -> bool {
        matches!(self, AppError::Network(_))
    }

    /// Get error type description
    pub fn error_type(&self) -> &'static str {
        match self {
            AppError::Network(_) => "网络错误",
            AppError::Auth(_) => "认证错误",
            AppError::TokenExpired => "Token 过期",
            AppError::NotFound(_) => "资源不存在",
            AppError::ApiError(_) => "API 错误",
            AppError::Io(_) => "IO 错误",
            AppError::Config(_) => "配置错误",
            AppError::EpisodeParse(_) => "解析错误",
            AppError::Regex(_) => "正则错误",
            AppError::Validation(_) => "验证错误",
        }
    }

    /// Get error code if available
    pub fn error_code(&self) -> Option<i32> {
        match self {
            AppError::TokenExpired => Some(401),
            AppError::NotFound(_) => Some(404),
            _ => None,
        }
    }

    /// Convert from a boxed dyn Error to AppError
    pub fn from_boxed_error(err: Box<dyn std::error::Error>) -> Self {
        let err_str = err.to_string();
        // Check for token expired indicators
        if err_str.contains("401") || err_str.contains("Unauthorized") || err_str.contains("unauthorized") {
            return AppError::TokenExpired;
        }
        // Check for network error indicators
        if err_str.contains("network") || err_str.contains("connection") || err_str.contains("timeout")
            || err_str.contains("resolve") || err_str.contains("connect") || err_str.contains("dns") {
            return AppError::Network(err_str);
        }
        // Default to ApiError
        AppError::ApiError(err_str)
    }
}

impl From<std::io::Error> for AppError {
    fn from(err: std::io::Error) -> Self {
        let err_str = err.to_string();
        if err_str.contains("401") || err_str.contains("Unauthorized") {
            AppError::TokenExpired
        } else if err.kind() == std::io::ErrorKind::NotFound {
            AppError::NotFound(err_str)
        } else if err.kind() == std::io::ErrorKind::TimedOut
            || err.kind() == std::io::ErrorKind::ConnectionAborted
            || err.kind() == std::io::ErrorKind::ConnectionRefused
            || err.kind() == std::io::ErrorKind::ConnectionReset {
            AppError::Network(err_str)
        } else {
            AppError::Io(err_str)
        }
    }
}

impl From<regex::Error> for AppError {
    fn from(err: regex::Error) -> Self {
        AppError::Regex(err.to_string())
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
