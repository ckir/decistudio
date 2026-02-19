//! # DeciStudio WASM Entry Point
//! 
//! This module provides the WebAssembly bindings for the browser environment.

use wasm_bindgen::prelude::*;
use decistudio_client_core::{StorageManager, OperationMode};

slint::include_modules!();

/// The entry point for the WASM application initialized by the browser.
/// // Loads data from LocalStorage and sets up the reactive UI bridge.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Instantiate UI
    let ui = AppWindow::new().map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    // Load initial data
    let data = StorageManager::load();

    // Map data to UI properties
    ui.set_ui_language(data.settings.ui_language.clone().into());
    ui.set_is_public_mode(data.settings.mode == OperationMode::Public);
    ui.set_server_url(data.settings.server_url.clone().into());
    ui.set_editor_content(data.last_session_content.clone().into());

    let ui_weak = ui.as_weak();

    // Registration of callback for persistence
    ui.on_settings_changed(move |lang, is_public, url| {
        if let Some(_) = ui_weak.upgrade() {
            let mut updated_data = StorageManager::load();
            updated_data.settings.ui_language = lang.to_string();
            updated_data.settings.mode = if is_public { OperationMode::Public } else { OperationMode::Local };
            updated_data.settings.server_url = url.to_string();
            // Save using platform abstraction
            let _ = StorageManager::save(&updated_data);
        }
    });

    // Run Slint loop
    ui.run().map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}