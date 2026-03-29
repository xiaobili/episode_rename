use crate::api::types::*;
use crate::error::{AppError, Result};
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;

#[derive(Debug, Clone)]
pub struct OpenListClient {
    base_url: String,
    token: Option<String>,
    client: Client,
}

impl OpenListClient {
    pub fn new(base_url: String, token: Option<String>) -> Self {
        Self {
            base_url,
            token,
            client: Client::new(),
        }
    }

    fn headers(&self) -> reqwest::header::HeaderMap {
        let mut h = reqwest::header::HeaderMap::new();
        h.insert("Content-Type", "application/json".parse().unwrap());
        if let Some(t) = &self.token {
            h.insert("Authorization", t.parse().unwrap());
        }
        h
    }

    async fn handle_response<T: DeserializeOwned>(&self, resp: Response) -> Result<T> {
        // Check for 401 Unauthorized first
        if resp.status() == StatusCode::UNAUTHORIZED {
            return Err(AppError::TokenExpired);
        }

        let text = resp.text().await.map_err(|e| {
            let err_str = e.to_string();
            if err_str.contains("401") || err_str.contains("Unauthorized") {
                AppError::TokenExpired
            } else if err_str.contains("connection")
                || err_str.contains("timeout")
                || err_str.contains("resolve")
            {
                AppError::Network(err_str)
            } else {
                AppError::ApiError(format!("读取响应失败：{}", e))
            }
        })?;

        let v: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
        if let Some(code) = v.get("code").and_then(|c| c.as_i64()) {
            if code != 200 {
                let msg = v
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("未知错误");
                // Check for 401 code in response body
                if code == 401 {
                    return Err(AppError::TokenExpired);
                }
                return Err(AppError::ApiError(msg.to_string()));
            }
        }
        serde_json::from_str(&text).map_err(|e| AppError::ApiError(format!("解析失败：{}", e)))
    }

    pub async fn login(&self, username: &str, password: &str) -> Result<String> {
        let url = format!("{}/api/auth/login", self.base_url);
        let resp = self
            .client
            .post(&url)
            .json(&LoginRequest {
                username: username.to_string(),
                password: password.to_string(),
            })
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("401") || err_str.contains("Unauthorized") {
                    AppError::TokenExpired
                } else if err_str.contains("connection")
                    || err_str.contains("timeout")
                    || err_str.contains("resolve")
                    || err_str.contains("dns")
                {
                    AppError::Network(err_str)
                } else {
                    AppError::Network(err_str)
                }
            })?;

        // Parse response as generic Value first to handle different formats
        let v: serde_json::Value = self.handle_response::<serde_json::Value>(resp).await?;

        // Try to extract token from different possible locations
        let token = v
            .get("data")
            .and_then(|data| {
                // Try data.token format
                data.get("token").and_then(|t| t.as_str())
            })
            .or_else(|| {
                // Try data as direct string format
                v.get("data").and_then(|d| d.as_str())
            })
            .or_else(|| {
                // Try token at root level
                v.get("token").and_then(|t| t.as_str())
            });

        token
            .map(|s| s.to_string())
            .ok_or_else(|| AppError::Auth("未返回 token".into()))
    }

    pub async fn list_directory(&self, path: &str) -> Result<Vec<FileItem>> {
        let url = format!("{}/api/fs/list", self.base_url);
        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&ListRequest { path: path.into() })
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("401") || err_str.contains("Unauthorized") {
                    AppError::TokenExpired
                } else if err_str.contains("connection")
                    || err_str.contains("timeout")
                    || err_str.contains("resolve")
                    || err_str.contains("dns")
                {
                    AppError::Network(err_str)
                } else {
                    AppError::Network(err_str)
                }
            })?;
        let api: ApiResponse<serde_json::Value> = self.handle_response(resp).await?;
        let content = api.data.and_then(|d| d.get("content").cloned());

        // Handle null or missing content as empty array
        match content {
            Some(value) if !value.is_null() => serde_json::from_value(value)
                .map_err(|e| AppError::ApiError(format!("解析失败：{}", e))),
            _ => Ok(Vec::new()),
        }
    }

    pub async fn batch_rename(&self, src_dir: &str, renames: Vec<RenameObject>) -> Result<()> {
        let url = format!("{}/api/fs/batch_rename", self.base_url);
        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&BatchRenameRequest {
                src_dir: src_dir.into(),
                rename_objects: renames,
            })
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("401") || err_str.contains("Unauthorized") {
                    AppError::TokenExpired
                } else if err_str.contains("connection")
                    || err_str.contains("timeout")
                    || err_str.contains("resolve")
                    || err_str.contains("dns")
                {
                    AppError::Network(err_str)
                } else {
                    AppError::Network(err_str)
                }
            })?;
        self.handle_response::<serde_json::Value>(resp).await?;
        Ok(())
    }

    pub async fn rename_single(&self, path: &str, new_name: &str) -> Result<()> {
        let url = format!("{}/api/fs/rename", self.base_url);
        let resp = self
            .client
            .post(&url)
            .headers(self.headers())
            .json(&RenameRequest {
                path: path.into(),
                name: new_name.into(),
            })
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("401") || err_str.contains("Unauthorized") {
                    AppError::TokenExpired
                } else if err_str.contains("connection")
                    || err_str.contains("timeout")
                    || err_str.contains("resolve")
                    || err_str.contains("dns")
                {
                    AppError::Network(err_str)
                } else {
                    AppError::Network(err_str)
                }
            })?;
        self.handle_response::<serde_json::Value>(resp).await?;
        Ok(())
    }

    pub async fn get_current_user(&self) -> Result<UserInfo> {
        let url = format!("{}/api/me", self.base_url);
        let resp = self
            .client
            .get(&url)
            .headers(self.headers())
            .send()
            .await
            .map_err(|e| {
                let err_str = e.to_string();
                if err_str.contains("401") || err_str.contains("Unauthorized") {
                    AppError::TokenExpired
                } else if err_str.contains("connection")
                    || err_str.contains("timeout")
                    || err_str.contains("resolve")
                    || err_str.contains("dns")
                {
                    AppError::Network(err_str)
                } else {
                    AppError::Network(err_str)
                }
            })?;
        let api: ApiResponse<UserInfo> = self.handle_response(resp).await?;
        api.data
            .ok_or_else(|| AppError::Auth("获取用户失败".into()))
    }
}
