//! # DeciStudio Native Entry Point
//! 
//! Desktop application entry point. Manages window state and filesystem I/O.

use slint::ComponentHandle;
use decistudio_client_core::{StorageManager, OperationMode};

slint::include_modules!();

/// The main entry point for the native desktop application.
/// // Handles initialization, maximization, and filesystem persistence.
fn main() -> Result<(), slint::PlatformError> {
    // Instantiate UI
    let ui = AppWindow::new()?;
    
    // Load local storage file (storage.json)
    let data = StorageManager::load();

    // Initialize UI properties
    ui.set_ui_language(data.settings.ui_language.clone().into());
    ui.set_is_public_mode(data.settings.mode == OperationMode::Public);
    ui.set_server_url(data.settings.server_url.clone().into());
    ui.set_editor_content(data.last_session_content.clone().into());

    let ui_weak = ui.as_weak();

    // Callback to save data whenever settings are modified
    ui.on_settings_changed(move |lang, is_public, url| {
        let mut updated_data = StorageManager::load();
        updated_data.settings.ui_language = lang.to_string();
        updated_data.settings.mode = if is_public { OperationMode::Public } else { OperationMode::Local };
        updated_data.settings.server_url = url.to_string();
        // Save to filesystem
        let _ = StorageManager::save(&updated_data);
    });

    // Auto-maximize window for a proper desktop feel
    let ui_maximize = ui.as_weak();
    slint::invoke_from_event_loop(move || {
        if let Some(ui) = ui_maximize.upgrade() {
            ui.window().set_maximized(true);
        }
    }).expect("Failed to synchronize with event loop");

    // Start UI
    ui.run()
}