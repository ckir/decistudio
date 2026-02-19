// Example: How to call the API stub from client-core

use decistudio_client_core::api;

pub fn demo_api() {
    if let Err(e) = api::ping() {
        eprintln!("API ping failed: {e}");
    }
}
