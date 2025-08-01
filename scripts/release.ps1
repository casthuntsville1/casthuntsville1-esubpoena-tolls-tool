# PowerShell script to create a GitHub release

param(
    [Parameter(Mandatory=$true)]
    [string]$Version,
    [string]$GitHubRepo = "your-username/esubpoena-tolls-tool"
)

Write-Host "Creating GitHub release for version $Version" -ForegroundColor Green

# Build the project
Write-Host "Building project..." -ForegroundColor Yellow
cargo build --release

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    exit 1
}

# Create installer
Write-Host "Creating installer..." -ForegroundColor Yellow
& ".\installer\create_installer.ps1" -Version $Version

if ($LASTEXITCODE -ne 0) {
    Write-Host "Installer creation failed!" -ForegroundColor Red
    exit 1
}

# Create git tag
Write-Host "Creating git tag v$Version..." -ForegroundColor Yellow
git tag -a "v$Version" -m "Release version $Version"
git push origin "v$Version"

# Create GitHub release using GitHub CLI
Write-Host "Creating GitHub release..." -ForegroundColor Yellow

$releaseNotes = @"
# eSubpoena Tolls Tool v$Version

## What's New
- High-performance Rust implementation
- Fast XML parsing and analytics
- Professional Excel export
- Modern GUI interface

## Installation
1. Download the ZIP file below
2. Extract the contents
3. Run `install.bat` to install to your desktop
4. Or run the executable directly

## Features
- Parse telecommunication XML data
- Generate comprehensive analytics
- Export to Excel with multiple worksheets
- Multi-file support
- Target number analysis
- Common contacts detection

## System Requirements
- Windows 10/11 (64-bit)
- No additional dependencies required

## Documentation
See README_RUST.md for detailed usage instructions.
"@

# Check if GitHub CLI is installed
try {
    gh --version | Out-Null
    Write-Host "GitHub CLI found, creating release..." -ForegroundColor Green
    
    $zipPath = "installer\output\esubpoena-tolls-tool-v$Version-windows.zip"
    
    gh release create "v$Version" `
        --title "eSubpoena Tolls Tool v$Version" `
        --notes "$releaseNotes" `
        --repo "$GitHubRepo" `
        "$zipPath"
        
    Write-Host "✓ GitHub release created successfully!" -ForegroundColor Green
    Write-Host "Release URL: https://github.com/$GitHubRepo/releases/tag/v$Version" -ForegroundColor Cyan
    
} catch {
    Write-Host "GitHub CLI not found. Please install it or create the release manually." -ForegroundColor Yellow
    Write-Host "Download URL: https://cli.github.com/" -ForegroundColor Cyan
    Write-Host "Manual release URL: https://github.com/$GitHubRepo/releases/new" -ForegroundColor Cyan
    Write-Host "Upload the ZIP file: installer\output\esubpoena-tolls-tool-v$Version-windows.zip" -ForegroundColor Cyan
}

Write-Host "`nRelease process completed!" -ForegroundColor Green 