# PowerShell script to create a Windows installer for eSubpoena Tolls Tool

param(
    [string]$Version = "1.0.0",
    [string]$SourceDir = "target/release",
    [string]$OutputDir = "installer/output"
)

Write-Host "Creating installer for eSubpoena Tolls Tool v$Version" -ForegroundColor Green

# Create output directory
if (!(Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force
}

# Copy executable
$exePath = Join-Path $SourceDir "esubpoena-tolls-tool.exe"
$destPath = Join-Path $OutputDir "esubpoena-tolls-tool.exe"

if (Test-Path $exePath) {
    Copy-Item $exePath $destPath -Force
    Write-Host "✓ Copied executable" -ForegroundColor Green
} else {
    Write-Host "✗ Executable not found at $exePath" -ForegroundColor Red
    exit 1
}

# Create installer batch file
$installerContent = @"
@echo off
echo ========================================
echo eSubpoena Tolls Tool - Installer
echo ========================================
echo.

set "INSTALL_DIR=%USERPROFILE%\Desktop\eSubpoena Tolls Tool"
set "EXE_NAME=esubpoena-tolls-tool.exe"

echo Installing to: %INSTALL_DIR%
echo.

REM Create installation directory
if not exist "%INSTALL_DIR%" mkdir "%INSTALL_DIR%"

REM Copy executable
copy "%EXE_NAME%" "%INSTALL_DIR%\%EXE_NAME%"

REM Create desktop shortcut
echo Creating desktop shortcut...
powershell -Command "`$WshShell = New-Object -comObject WScript.Shell; `$Shortcut = `$WshShell.CreateShortcut('%USERPROFILE%\Desktop\eSubpoena Tolls Tool.lnk'); `$Shortcut.TargetPath = '%INSTALL_DIR%\%EXE_NAME%'; `$Shortcut.Save()"

echo.
echo ========================================
echo Installation Complete!
echo ========================================
echo.
echo The application has been installed to:
echo %INSTALL_DIR%
echo.
echo A shortcut has been created on your desktop.
echo.
echo You can now run the application by:
echo 1. Double-clicking the desktop shortcut, or
echo 2. Running: %INSTALL_DIR%\%EXE_NAME%
echo.
pause
"@

$installerPath = Join-Path $OutputDir "install.bat"
$installerContent | Out-File -FilePath $installerPath -Encoding ASCII
Write-Host "✓ Created installer script" -ForegroundColor Green

# Create uninstaller
$uninstallerContent = @"
@echo off
echo ========================================
echo eSubpoena Tolls Tool - Uninstaller
echo ========================================
echo.

set "INSTALL_DIR=%USERPROFILE%\Desktop\eSubpoena Tolls Tool"
set "EXE_NAME=esubpoena-tolls-tool.exe"
set "SHORTCUT=%USERPROFILE%\Desktop\eSubpoena Tolls Tool.lnk"

echo This will remove eSubpoena Tolls Tool from your system.
echo.
echo Files to be removed:
echo - %INSTALL_DIR%
echo - %SHORTCUT%
echo.

set /p "confirm=Are you sure you want to continue? (y/N): "
if /i not "%confirm%"=="y" goto :end

echo.
echo Removing application files...

REM Remove desktop shortcut
if exist "%SHORTCUT%" del "%SHORTCUT%"

REM Remove installation directory
if exist "%INSTALL_DIR%" rmdir /s /q "%INSTALL_DIR%"

echo.
echo ========================================
echo Uninstallation Complete!
echo ========================================
echo.
echo eSubpoena Tolls Tool has been removed from your system.
echo.

:end
pause
"@

$uninstallerPath = Join-Path $OutputDir "uninstall.bat"
$uninstallerContent | Out-File -FilePath $uninstallerPath -Encoding ASCII
Write-Host "✓ Created uninstaller script" -ForegroundColor Green

# Copy documentation
$docs = @("README_RUST.md", "RUST_SETUP_GUIDE.md", "CONVERSION_SUMMARY.md")
foreach ($doc in $docs) {
    if (Test-Path $doc) {
        Copy-Item $doc $OutputDir -Force
        Write-Host "✓ Copied $doc" -ForegroundColor Green
    }
}

# Create README for installer
$installerReadme = @"
# eSubpoena Tolls Tool - Installer

## Quick Installation

1. **Download** the installer files
2. **Run** `install.bat` to install the application
3. **Use** the desktop shortcut to launch the application

## Manual Installation

If you prefer to install manually:

1. Copy `esubpoena-tolls-tool.exe` to your desired location
2. Run the executable directly

## Uninstallation

To remove the application:

1. Run `uninstall.bat` to remove all files and shortcuts
2. Or manually delete the installation directory

## System Requirements

- Windows 10/11 (64-bit)
- No additional dependencies required
- ~50MB disk space

## Features

- Parse telecommunication XML data
- Generate comprehensive analytics
- Export to Excel with multiple worksheets
- Multi-file support
- Target number analysis
- Common contacts detection

## Support

For issues or questions, see the documentation files included in this package.
"@

$installerReadmePath = Join-Path $OutputDir "README_INSTALLER.md"
$installerReadme | Out-File -FilePath $installerReadmePath -Encoding UTF8
Write-Host "✓ Created installer README" -ForegroundColor Green

# Create ZIP file
$zipName = "esubpoena-tolls-tool-v$Version-windows.zip"
$zipPath = Join-Path $OutputDir $zipName

Write-Host "Creating ZIP file..." -ForegroundColor Yellow
Compress-Archive -Path "$OutputDir\*" -DestinationPath $zipPath -Force
Write-Host "✓ Created ZIP file: $zipName" -ForegroundColor Green

Write-Host "`nInstallation package created successfully!" -ForegroundColor Green
Write-Host "Location: $OutputDir" -ForegroundColor Cyan
Write-Host "ZIP file: $zipPath" -ForegroundColor Cyan 