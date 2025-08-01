@echo off
echo ========================================
echo eSubpoena Tolls Tool - Simple Version
echo ========================================
echo.

echo Installing required dependency (openpyxl)...
py -m pip install openpyxl

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Failed to install openpyxl! Please check if Python is installed.
    echo You can install Python from: https://www.python.org/downloads/
    pause
    exit /b 1
)

echo.
echo Dependency installed successfully!
echo.
echo Starting the application...
echo.

py esubpoena_tolls_tool_simple.py

echo.
echo Application finished.
pause 