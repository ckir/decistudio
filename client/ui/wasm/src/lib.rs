use wasm_bindgen::prelude::*;

// This macro includes the compiled UI from the .slint file
slint::include_modules!();

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    // This matches the shared UI used in native builds [cite: 36]
    let ui = AppWindow::new().map_err(|e| JsValue::from_str(&e.to_string()))?;
    
    ui.on_request_increase_value({
        let ui_handle = ui.as_weak();
        move || {
            let ui = ui_handle.unwrap();
            ui.set_counter(ui.get_counter() + 1); // Logic for the counter [cite: 37]
        }
    });

    // This call will now find the <canvas id="canvas"> 
    ui.run().map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(())
}