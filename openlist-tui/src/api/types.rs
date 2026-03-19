use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: Option<String>,
    pub data: Option<T>,
}

#[derive(Debug, Clone, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ListRequest {
    pub path: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileItem {
    pub name: String,
    pub is_dir: bool,
    pub size: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
pub struct BatchRenameRequest {
    pub src_dir: String,
    pub rename_objects: Vec<RenameObject>,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenameObject {
    pub src_name: String,
    pub new_name: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct RenameRequest {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct UserInfo {
    pub username: String,
    pub nick: Option<String>,
}
