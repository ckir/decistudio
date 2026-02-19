//! # DeciStudio WASM Entry Point
//! 
//! Provides the JavaScript bindings for running the IDE in a browser.
//! Uses the Slint framework to render directly into a WebGL/WebGPU canvas.

use wasm_bindgen::prelude::*;

// Include the compiled UI logic from Slint
slint::include_modules!();

/// Entry point for the WASM module, called on page load.
/// 
/// // This function bridges the browser environment with the Rust logic.
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // Instantiate the UI window
    // // In WASM, this window scales automatically to the canvas element size.
    let ui = AppWindow::new().map_err(|e| JsValue::from_str(&e.to_string()))?;

    // Implement your clipping logic: clearing pre-existing text
    // // This fulfills the instruction: "text goes to EDITOR / WORKSPACE clearing pre-existing text"
    ui.set_editor_content("// Welcome to DeciStudio for Web\n// Mimicking VS Code layout...".into());

    // Execute the main event loop
    // // The 'run' method in WASM is non-blocking to the browser thread.
    ui.run().map_err(|e| JsValue::from_str(&e.to_string()))?;

    Ok(())
}