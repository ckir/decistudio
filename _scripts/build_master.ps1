# ==============================================================================
# DeciStudio Master Build & Run Menu (Fail-Fast)
# ==============================================================================
$ErrorActionPreference = "Stop"

function Fail {
    param($Message)
    Write-Host "`n❌ FATAL ERROR: $Message" -ForegroundColor Red
    exit 1
}

function Detect-ProjectRoot {
    $current = Get-Location
    while ($true) {
        if (Test-Path "Cargo.toml" -PathType Leaf) { return (Get-Location).Path }
        if ($current.Path -eq $current.Path.Substring(0,3)) { Fail "Root not found." }
        Set-Location ..
    }
}

$ProjectRoot = Detect-ProjectRoot
function Path-Abs { param($Relative) return Join-Path $ProjectRoot $Relative }

# ------------------------------------------------------------------------------
# BUILD LOGIC
# ------------------------------------------------------------------------------

function Build-Native {
    Write-Host "`n--- Building Native Client ---" -ForegroundColor Cyan
    cargo build -p decistudio-client-ui-native --release
    if ($LASTEXITCODE -ne 0) { Fail "Native build failed." }
}

function Build-WASM {
    Write-Host "`n--- Building WASM Client ---" -ForegroundColor Cyan
    $wasmTargetDir = Path-Abs "target/wasm-web-dist"
    
    # Push into WASM dir to ensure build.rs runs correctly
    Push-Location (Path-Abs "client/ui/wasm")
    cargo build --target wasm32-unknown-unknown --release
    if ($LASTEXITCODE -ne 0) { Pop-Location; Fail "WASM compilation failed." }
    Pop-Location

    # Prepare distribution folder in target
    $wasmFile = Path-Abs "target/wasm32-unknown-unknown/release/decistudio_client_ui_wasm.wasm"
    if (Test-Path $wasmTargetDir) { Remove-Item $wasmTargetDir -Recurse -Force }
    New-Item -ItemType Directory -Path $wasmTargetDir | Out-Null

    Write-Host "Generating Bindings..."
    wasm-bindgen --target web --out-dir $wasmTargetDir --no-typescript $wasmFile
    
# 3. Create HTML Runner with Canvas
# Update this section within your Build-WASM function in _scripts\build_master.ps1
    @"
<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <title>DeciStudio WASM</title>
    <style>
        body { background: #121212; margin: 0; display: flex; justify-content: center; align-items: center; height: 100vh; }
        /* The canvas must have an ID for Slint to find it */
        #canvas { width: 100%; height: 100%; display: block; }
    </style>
</head>
<body>
    <canvas id="canvas"></canvas>
    <script type="module">
        import init from './decistudio_client_ui_wasm.js';
        init().catch(console.error);
    </script>
</body>
</html>
"@ | Out-File "$wasmTargetDir/index.html" -Encoding utf8
}

# ------------------------------------------------------------------------------
# MENU SYSTEM
# ------------------------------------------------------------------------------

while ($true) {
    Clear-Host
    Write-Host "===================================================="
    Write-Host "           DECISTUDIO MASTER CONTROL"
    Write-Host "===================================================="
    Write-Host " 0) Check Environment (Rust, WASM, Bindgen)"
    Write-Host " 1) [Native] Build & Run"
    Write-Host " 2) [WASM]   Build Only"
    Write-Host " 3) [WASM]   Build & Serve (Port 8000)"
    Write-Host " 4) [Server] Build & Run (Standalone)"
    Write-Host " 5) [Tools]  Sync Translations (EN -> EL)"
    Write-Host " 6) [Repair] Deep Clean & Update (Fixes Compiler Errors)"
    Write-Host " 7) Exit"
    Write-Host "===================================================="
    
    $choice = Read-Host "Select an option"
    switch ($choice) {
        "0" { 
            Write-Host "Checking versions..."
            cargo --version; rustup target list --installed; wasm-bindgen --version
            Pause 
        }
        "1" { Build-Native; Start-Process (Path-Abs "target/release/decistudio-client-ui-native.exe"); Pause }
        "2" { Build-WASM; Pause }
        "3" { Build-WASM; python -m http.server 8000 --directory (Path-Abs "target/wasm-web-dist") }
        "4" { cargo build -p decistudio-server-standalone; Start-Process (Path-Abs "target/release/decistudio-server-standalone.exe") -NoNewWindow -Wait; Pause }
        "5" { 
            # Translation sync logic
            $en = Get-Content (Path-Abs "client/translations/en/ui.json") | ConvertFrom-Json
            $el = Get-Content (Path-Abs "client/translations/el/ui.json") | ConvertFrom-Json
            foreach($k in $en.PSObject.Properties.Name){ if(-not $el.$k){ $el | Add-Member $k "[TODO]" } }
            $el | ConvertTo-Json | Out-File (Path-Abs "client/translations/el/ui.json") -Encoding utf8
            Write-Host "Synced."; Pause
        }
        "6" { 
            Write-Host "Resetting workspace..." -ForegroundColor Yellow
            Remove-Item (Path-Abs "Cargo.lock") -ErrorAction SilentlyContinue
            cargo clean
            cargo update
            Write-Host "Done. Try building now." -ForegroundColor Green; Pause
        }
        "7" { exit 0 }
    }
}