use crate::api::types::FileItem;

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)]
pub enum Focus {
    Directory,
    File,
    Input,
}

pub struct NavigationState {
    pub current_path: String,
    pub path_history: Vec<String>,
    pub directories: Vec<FileItem>,
    pub files: Vec<FileItem>,
    pub selected_index: usize,
    pub focus: Focus,
    /// Directory name to select after navigating to parent directory.
    /// Stores the child directory name before navigation so that after
    /// loading parent contents, we can point the selection to it.
    pub pending_select_dir: Option<String>,
}

impl Default for NavigationState {
    fn default() -> Self {
        Self {
            current_path: "/".into(),
            path_history: vec![],
            directories: vec![],
            files: vec![],
            selected_index: 0,
            focus: Focus::Directory,
            pending_select_dir: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_navigation_state_default() {
        let nav = NavigationState::default();
        assert!(nav.current_path.is_empty() || nav.current_path == "/");
        assert!(nav.directories.is_empty());
        assert!(nav.files.is_empty());
        assert_eq!(nav.selected_index, 0);
        assert!(nav.pending_select_dir.is_none());
    }

    #[test]
    fn test_navigation_state_fields_accessible() {
        let mut nav = NavigationState::default();
        nav.current_path = "/test".to_string();
        nav.selected_index = 5;
        assert_eq!(nav.current_path, "/test");
        assert_eq!(nav.selected_index, 5);
    }
}
