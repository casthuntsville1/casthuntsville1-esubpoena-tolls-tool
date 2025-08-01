@echo off
echo Setting up Visual Studio environment and building project...

REM Set up Visual Studio environment
call "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"

REM Build the project
cargo build --release

if %errorlevel% equ 0 (
    echo.
    echo ========================================
    echo Build successful!
    echo ========================================
    echo.
    echo The executable is located at:
    echo target\release\esubpoena-tolls-tool.exe
    echo.
    pause
) else (
    echo.
    echo ========================================
    echo Build failed!
    echo ========================================
    echo.
    pause
) 