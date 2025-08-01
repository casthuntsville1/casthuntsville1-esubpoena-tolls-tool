@echo off
echo ========================================
echo eSubpoena Tolls Tool - Build and Run
echo ========================================
echo.

echo Building the application...
cargo build --release

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Build failed! Please check the error messages above.
    echo Make sure Rust is properly installed.
    pause
    exit /b 1
)

echo.
echo Build successful! Running the application...
echo.

cargo run --release

echo.
echo Application finished.
pause 