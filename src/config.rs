use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use crate::error::{AppError, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub base_url: String,
    pub token: Option<String>,
    pub username: Option<String>,
    pub use_nerdfont: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_url: "http://192.168.1.1:5244".to_string(),
            token: None,
            username: None,
            use_nerdfont: true,
        }
    }
}

impl Config {
    pub fn config_dir() -> Result<PathBuf> {
        let dir = dirs::config_dir()
            .ok_or_else(|| AppError::Config("无法获取配置目录".to_string()))?;
        Ok(dir.join("openlist-tui"))
    }

    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.json"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path)?;
        serde_json::from_str(&content)
            .map_err(|e| AppError::Config(format!("解析配置失败：{}", e)))
    }

    pub fn save(&self) -> Result<()> {
        let dir = Self::config_dir()?;
        fs::create_dir_all(&dir)?;
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| AppError::Config(format!("序列化配置失败：{}", e)))?;
        fs::write(Self::config_path()?, content)?;
        Ok(())
    }
}
