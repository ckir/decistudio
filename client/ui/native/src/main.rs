// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

// ------------------------------------------------------------
// Example: How to use translations (from client-core)
// ------------------------------------------------------------
// use crate::example_translations::demo_translations;
// demo_translations();

// ------------------------------------------------------------
// Example: How to call the API stub (from client-core)
// ------------------------------------------------------------
// use crate::example_api::demo_api;
// demo_api();
// ------------------------------------------------------------


slint::include_modules!();

#[cfg_attr(target_arch = "wasm32",
           wasm_bindgen::prelude::wasm_bindgen(start))]
fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1);
        }
    });

    ui.run()?;

    Ok(())
}
