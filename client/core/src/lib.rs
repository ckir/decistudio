use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// A simple translation map loaded from JSON.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranslationMap {
    pub entries: HashMap<String, String>,
}

/// Translator struct used by the UI.
pub struct Translator {
    map: HashMap<String, String>,
}

impl Translator {
    pub fn from_map(map: HashMap<String, String>) -> Self {
        Self { map }
    }

    pub fn t(&self, key: &str) -> String {
        self.map.get(key).cloned().unwrap_or_else(|| key.to_string())
    }
}

/// Load translations from client/translations/<locale>/ui.json
pub fn load_translations(locale: &str) -> Result<Translator> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.pop(); // core/
    path.push("translations");
    path.push(locale);
    path.push("ui.json");

    let data = fs::read_to_string(&path)?;
    let entries: HashMap<String, String> = serde_json::from_str(&data)?;
    Ok(Translator::from_map(entries))
}

/// Stub API module — will be replaced by generated API bindings later.
pub mod api {
    use anyhow::Result;

    /// Example API call stub.
    pub fn ping() -> Result<()> {
        println!("decistudio-client-core::api::ping() stub called");
        Ok(())
    }
}
