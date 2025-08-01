# Rust Setup Guide for eSubpoena Tolls Tool

## Prerequisites
1. **Visual Studio Build Tools 2022** (already installed)
2. **Windows 10/11** (confirmed)

## Step 1: Install Rust

### Option A: Using the Official Installer
1. Go to https://rustup.rs/
2. Download the Windows installer
3. Run the installer and follow the prompts
4. Choose the default installation (recommended)

### Option B: Using winget (if available)
```powershell
winget install Rustlang.Rust.MSVC
```

### Option C: Manual Installation
1. Download rustup-init.exe from https://win.rustup.rs/
2. Run: `rustup-init.exe --default-toolchain stable --profile default -y`

## Step 2: Verify Installation
After installation, restart your terminal/PowerShell and run:
```powershell
rustc --version
cargo --version
```

## Step 3: Install rust-analyzer in Cursor
1. Open Cursor
2. Go to Extensions (Ctrl+Shift+X)
3. Search for "rust-analyzer"
4. Install the official rust-analyzer extension
5. Restart Cursor

## Step 4: Build the Project
Once Rust is installed, run:
```powershell
cargo build
```

## Troubleshooting

### If rustc is not recognized:
1. Restart your terminal/PowerShell
2. Check if Rust is in your PATH: `echo $env:PATH`
3. Manually add Rust to PATH if needed:
   ```powershell
   $env:PATH += ";$env:USERPROFILE\.cargo\bin"
   ```

### If Visual Studio Build Tools are missing:
1. Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
2. Install with C++ build tools workload

### If you get linker errors:
1. Ensure Visual Studio Build Tools are installed
2. Run: `rustup default stable-msvc`
3. Restart terminal and try again

## Project Structure
The Rust project is already set up with:
- `src/main.rs` - Application entry point
- `src/app.rs` - GUI application logic
- `src/data_models.rs` - Data structures
- `src/xml_parser.rs` - XML parsing
- `src/analytics.rs` - Analytics calculations
- `src/excel_exporter.rs` - Excel export functionality

## Next Steps
Once Rust is properly installed:
1. Run `cargo build` to compile the project
2. Run `cargo run` to start the application
3. The application will have the same functionality as the Python version but with better performance 