use crate::api::client::OpenListClient;
use crate::config::Config;
use crate::state::{AsyncState, AuthState, NavigationState, RenameState, UIState};

// Re-export for backward compatibility with tests and render
// These are used by external test code via openlist_tui::app::*
#[allow(unused_imports)]
pub use crate::state::{
    ErrorInfo, Focus, LoginFocus, RegexFocus, RenameMode, Screen, UnifiedFocus,
};

pub struct App {
    pub client: OpenListClient, // D-06: top-level
    pub config: Config,         // D-07: top-level
    pub navigation: NavigationState,
    pub auth: AuthState,
    pub rename: RenameState,
    pub ui: UIState,
    pub async_state: AsyncState, // D-08
}

impl Default for App {
    fn default() -> Self {
        Self {
            client: OpenListClient::new("http://192.168.1.1:5244".into(), None),
            config: Config::default(),
            navigation: NavigationState::default(),
            auth: AuthState::default(),
            rename: RenameState::default(),
            ui: UIState::default(),
            async_state: AsyncState::default(),
        }
    }
}

impl App {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_config(config: Config) -> Self {
        let base_url = config.base_url.clone();
        let token = config.token.clone();
        Self {
            client: OpenListClient::new(base_url, token),
            config,
            ..Self::default()
        }
    }

    // ========================================================================
    // Essential wrappers for main.rs compatibility (async or cross-module)
    // ========================================================================

    /// Async wrapper for main.rs directory loading.
    pub async fn load_directory_contents(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        crate::update::load_directory_contents(self).await
    }

    /// Error wrapper for main.rs error handling.
    pub fn handle_api_error(&mut self, error: Box<dyn std::error::Error + 'static>) {
        crate::update::handle_api_error(self, error);
    }

    // ========================================================================
    // Simple predicates used by input.rs and tasks.rs
    // ========================================================================

    /// Check if regex preview is available (non-empty and no error).
    pub fn has_regex_preview(&self) -> bool {
        !self.rename.regex.preview.is_empty() && self.rename.regex.error.is_none()
    }

    // ========================================================================
    // Result accessors for tasks.rs
    // ========================================================================

    /// Take smart rename results, clearing the internal storage.
    pub fn take_smart_rename_results(&mut self) -> Vec<(String, String, bool)> {
        std::mem::take(&mut self.rename.smart.results)
    }

    /// Take manual rename results, clearing the internal storage.
    pub fn take_manual_rename_results(&mut self) -> Vec<(String, String, bool)> {
        std::mem::take(&mut self.rename.manual.results)
    }

    /// Take unified rename results, clearing the internal storage.
    pub fn take_unified_rename_results(&mut self) -> Vec<(String, String, bool)> {
        std::mem::take(&mut self.rename.unified.results)
    }

    /// Take regex rename results, clearing the internal storage.
    pub fn take_regex_rename_results(&mut self) -> Vec<(String, String, bool)> {
        std::mem::take(&mut self.rename.regex.results)
    }

    // ========================================================================
    // Simple accessors for tests
    // ========================================================================

    /// Get the current single rename target file.
    #[allow(dead_code)]
    pub fn get_single_rename_target(&self) -> Option<&crate::api::types::FileItem> {
        self.rename.single.target.as_ref()
    }
}
