# DeciStudio

A high-performance, cross-platform Rust application framework built with **Slint 1.15.1**. This repository implements a unified UI architecture, sharing a single codebase across Native Desktop and WebAssembly (WASM) targets, supported by a modular server-side infrastructure.

## 🏗 Project Architecture

DeciStudio is organized as a Rust workspace to ensure clean separation between platform-specific implementations while maximizing core logic reuse.

* **`client/ui/native`**: The primary UI crate. It contains the desktop entry point and holds the master `.slint` definitions used across all platforms.
* **`client/ui/wasm`**: The web client wrapper. It utilizes `wasm-bindgen` and references shared UI assets from the native crate to ensure visual parity.
* **`client/core`**: The shared client engine. Handles dynamic JSON-based localization (i18n) and shared data models for the UI.
* **`server/standalone`**: A robust server implementation with PostgreSQL integration via SQLx.
* **`server/stateless`**: A specialized server component designed for serverless environments (e.g., AWS Lambda).
* **`client/translations`**: Centralized repository for UI strings in English (`en`) and Greek (`el`).



---

## 🚀 Getting Started

### Prerequisites
Ensure your development environment is configured with the following:
* **Rust**: Latest stable version (Edition 2021).
* **WASM Target**: `rustup target add wasm32-unknown-unknown`.
* **Wasm-bindgen CLI**: `cargo install wasm-bindgen-cli`.
* **Python 3**: Required for serving the WASM distribution locally.

### Automation via Master Control
A centralized PowerShell script in `_scripts/build_master.ps1` automates the build pipeline, environment validation, and workspace maintenance.

1. Open PowerShell in the root directory.
2. Run the master control:

   ./_scripts/build_master.ps1
   3. **Core Options**:
   * **Option 1 (Native)**: Compiles the desktop client and launches the executable. Ideal for local development and high-performance testing.
   * **Option 3 (WASM)**: Compiles the UI for web32-unknown-unknown, generates JS bindings via `wasm-bindgen`, and starts a local Python server at `http://localhost:8000`.
   * **Option 5 (i18n)**: Synchronizes translation keys from the English source (`en/ui.json`) to the Greek target (`el/ui.json`), flagging missing entries with `[TODO]` tags.
   * **Option 6 (Repair)**: A critical utility that deep cleans the workspace, resets `Cargo.lock`, and forces a dependency update to resolve Slint internal compiler version drifts (e.g., `BuildDiagnostics` errors).

---

## 🛠 Development Deep Dive

### UI Synchronization & Rendering
The UI is defined once in `client/ui/native/ui/app-window.slint`. This single source of truth is shared across platforms:
* **Native**: Compiled directly via the native crate's `build.rs`.
* **WASM**: Rendered into a `<canvas id="canvas">`. The build script automatically handles the injection of this mandatory canvas into the distribution `index.html`.

### The WASM Event Loop
When inspecting the browser console, you will see the following message:
`Error: Using exceptions for control flow, don't mind me.`

**This is intended behavior.** It is the standard mechanism used by the `winit` crate (Slint's windowing backend) to hand over execution to the browser's event loop without blocking the main thread.

---

## 📄 License
This project is licensed under the **MIT License**.