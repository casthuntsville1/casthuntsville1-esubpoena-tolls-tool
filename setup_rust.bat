@echo off
echo ========================================
echo eSubpoena Tolls Tool - Rust Setup
echo ========================================

echo.
echo Checking if Rust is installed...
rustc --version >nul 2>&1
if %errorlevel% neq 0 (
    echo Rust is not installed or not in PATH.
    echo.
    echo Please install Rust using one of these methods:
    echo 1. Go to https://rustup.rs/ and download the installer
    echo 2. Run: winget install Rustlang.Rust.MSVC
    echo 3. Download rustup-init.exe from https://win.rustup.rs/
    echo.
    echo After installation, restart this script.
    pause
    exit /b 1
)

echo Rust is installed!
rustc --version
cargo --version

echo.
echo ========================================
echo Building the project...
echo ========================================

cargo build

if %errorlevel% neq 0 (
    echo.
    echo Build failed! Please check the error messages above.
    echo Make sure you have Visual Studio Build Tools installed.
    pause
    exit /b 1
)

echo.
echo ========================================
echo Build successful!
echo ========================================

echo.
echo To run the application:
echo cargo run
echo.
echo To build a release version:
echo cargo build --release
echo.
echo To install rust-analyzer in Cursor:
echo 1. Open Cursor
echo 2. Go to Extensions (Ctrl+Shift+X)
echo 3. Search for "rust-analyzer"
echo 4. Install the official rust-analyzer extension
echo 5. Restart Cursor
echo.

pause 