//! Input validation module for user inputs.
//!
//! This module provides validation functions for various input types.
//! Currently unused but kept for future input validation features.

#![allow(dead_code)]

use crate::error::{AppError, Result};

#[derive(Debug, Clone, Copy)]
pub enum InputType {
    ShowName,
    Season,
    Episode,
    Regex,
}

pub fn validate_input(input: &str, input_type: InputType) -> Result<()> {
    match input_type {
        InputType::ShowName => {
            if input.is_empty() {
                return Err(AppError::Validation("剧集名称不能为空".to_string()));
            }
        }
        InputType::Season => {
            if !input.chars().all(|c| c.is_numeric()) {
                return Err(AppError::Validation("季数必须是数字".to_string()));
            }
        }
        InputType::Episode => {
            if !input.chars().all(|c| c.is_numeric()) {
                return Err(AppError::Validation("集数必须是数字".to_string()));
            }
        }
        InputType::Regex => {
            regex::Regex::new(input)
                .map_err(|_| AppError::Validation("无效的正则表达式".to_string()))?;
        }
    }
    Ok(())
}

/// Validate folder name per D-04, D-05, D-06, D-07, D-08.
///
/// Returns Some(error_message) if validation fails, None if valid.
///
/// # Arguments
/// * `name` - The folder name to validate
/// * `existing_dirs` - List of existing directories to check for duplicates
///
/// # Validation Rules
/// - D-04: Not empty
/// - D-05: Max 255 characters
/// - D-06: Not a reserved name (Unix: "." ".."; Windows: CON, PRN, AUX, NUL, COM1-9, LPT1-9)
/// - D-07: Not a duplicate of existing directory
/// - D-08: No invalid characters (/ \ : * ? " < > |)
pub fn validate_folder_name(name: &str, existing_dirs: &[crate::api::types::FileItem]) -> Option<String> {
    // D-04: Empty check
    if name.is_empty() {
        return Some("文件夹名称不能为空".to_string());
    }

    // D-05: Length check (255 char filesystem limit)
    if name.len() > 255 {
        return Some("文件夹名称不能超过255个字符".to_string());
    }

    // D-06: Reserved names check
    let reserved_unix = [".", ".."];
    let reserved_windows = [
        "CON", "PRN", "AUX", "NUL",
        "COM1", "COM2", "COM3", "COM4", "COM5",
        "COM6", "COM7", "COM8", "COM9",
        "LPT1", "LPT2", "LPT3", "LPT4", "LPT5",
        "LPT6", "LPT7", "LPT8", "LPT9",
    ];

    if reserved_unix.contains(&name) {
        return Some("此名称为系统保留".to_string());
    }

    let name_upper = name.to_uppercase();
    if reserved_windows.contains(&name_upper.as_str()) {
        return Some("此名称为Windows系统保留".to_string());
    }

    // D-08: Invalid characters check
    let invalid_chars = ['/', '\\', ':', '*', '?', '"', '<', '>', '|'];
    if name.chars().any(|c| invalid_chars.contains(&c)) {
        return Some("名称包含无效字符: / \\ : * ? \" < > |".to_string());
    }

    // D-07: Duplicate check against existing directories
    if existing_dirs.iter().any(|d| d.name == name) {
        return Some("已存在同名文件夹".to_string());
    }

    None
}
