//! # DeciStudio Core Storage
//! 
//! This crate provides the shared data models and storage abstractions used by
//! both the Native and WASM client implementations.

use serde::{Serialize, Deserialize};

/// Defines the operational environment of the client.
#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq)]
pub enum OperationMode {
    /// Data is stored exclusively on the local machine or browser.
    Local,
    /// Data is synchronized via a remote REST API.
    Public,
}

/// User-configurable application settings.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppSettings {
    /// The language of the User Interface.
    pub ui_language: String,
    /// The current operation mode (Local or Public).
    pub mode: OperationMode,
    /// The remote server URL (relevant only in Public mode).
    pub server_url: String,
}

impl Default for AppSettings {
    /// Provides default values for first-time application startup.
    fn default() -> Self {
        Self {
            ui_language: "English".into(),
            mode: OperationMode::Local,
            server_url: "https://api.decistudio.com".into(),
        }
    }
}

/// The root data structure persisted to the platform database.
#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct AppData {
    /// Collection of text fragments saved by the user.
    pub clippings: Vec<String>,
    /// The last state of the editor workspace.
    pub last_session_content: String,
    /// Persistent application settings.
    pub settings: AppSettings,
}

/// Cross-platform storage manager for persisting application state.
pub struct StorageManager;

impl StorageManager {
    /// The unique key used for data storage identification.
    /// // Allowed dead_code to prevent warnings during native-only builds.
    #[allow(dead_code)]
    const STORAGE_KEY: &'static str = "decistudio_data";

    /// Persists AppData to the platform-specific database.
    /// 
    /// // Uses target-gate logic to switch between LocalStorage (WASM) and File System (Native).
    pub fn save(data: &AppData) -> Result<(), String> {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_storage::{LocalStorage, Storage};
            // Save to browser LocalStorage
            LocalStorage::set(Self::STORAGE_KEY, data).map_err(|e| e.to_string())
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            let path = Self::get_native_path()?;
            // Serialize to JSON for native readability
            let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
            // Write to local disk
            std::fs::write(path, json).map_err(|e| e.to_string())
        }
    }

    /// Retrieves AppData from the platform-specific database.
    /// 
    /// // Returns a default AppData instance if no existing storage is found.
    pub fn load() -> AppData {
        #[cfg(target_arch = "wasm32")]
        {
            use gloo_storage::{LocalStorage, Storage};
            // Retrieve from browser LocalStorage
            LocalStorage::get(Self::STORAGE_KEY).unwrap_or_default()
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            // Attempt to load from the native filesystem path
            if let Ok(path) = Self::get_native_path() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    return serde_json::from_str(&content).unwrap_or_default();
                }
            }
            AppData::default()
        }
    }

    /// Helper to resolve the system configuration path for Native builds.
    #[cfg(not(target_arch = "wasm32"))]
    fn get_native_path() -> Result<std::path::PathBuf, String> {
        use directories::ProjectDirs;
        // Define the project directory based on OS standards
        let dirs = ProjectDirs::from("com", "decistudio", "ide").ok_or("Path error")?;
        // Create directory if missing
        std::fs::create_dir_all(dirs.config_dir()).map_err(|e| e.to_string())?;
        Ok(dirs.config_dir().join("storage.json"))
    }
}