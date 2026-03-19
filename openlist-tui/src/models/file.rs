use crate::api::types::FileItem;

#[derive(Debug, Clone)]
pub struct File {
    pub name: String,
    pub size: u64,
    pub extension: String,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub name: String,
}

impl File {
    pub fn from_api_item(item: FileItem) -> Option<Self> {
        if item.is_dir {
            return None;
        }
        let ext = item.name.rsplit('.').next().unwrap_or("").to_lowercase();
        Some(Self {
            name: item.name,
            size: item.size.unwrap_or(0),
            extension: ext,
        })
    }

    pub fn is_video(&self) -> bool {
        [
            "mp4", "mkv", "avi", "mov", "wmv", "flv", "webm", "m4v", "mpg", "mpeg", "ts", "m2ts",
            "vob", "iso",
        ]
        .contains(&self.extension.as_str())
    }
}

impl Directory {
    pub fn from_api_item(item: FileItem) -> Option<Self> {
        if !item.is_dir {
            return None;
        }
        Some(Self { name: item.name })
    }
}
