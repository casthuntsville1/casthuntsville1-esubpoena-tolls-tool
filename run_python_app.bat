@echo off
echo ========================================
echo eSubpoena Tolls Tool - Python Version
echo ========================================
echo.

echo Installing Python dependencies...
pip install -r requirements.txt

if %ERRORLEVEL% NEQ 0 (
    echo.
    echo Failed to install dependencies! Please check if Python and pip are installed.
    echo You can install Python from: https://www.python.org/downloads/
    pause
    exit /b 1
)

echo.
echo Dependencies installed successfully!
echo.
echo Starting the application...
echo.

python esubpoena_tolls_tool.py

echo.
echo Application finished.
pause 