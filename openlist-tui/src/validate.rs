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
