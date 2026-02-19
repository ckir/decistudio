// Example: How to load translations from client-core

use decistudio_client_core::load_translations;

pub fn demo_translations() {
    match load_translations("en") {
        Ok(tr) => {
            println!("Example translation: {}", tr.t("app.greeting"));
        }
        Err(e) => {
            eprintln!("Failed to load translations: {e}");
        }
    }
}
