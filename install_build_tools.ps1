# PowerShell script to install Visual Studio Build Tools for Rust development

Write-Host "Installing Visual Studio Build Tools for Rust..." -ForegroundColor Green

# Download Visual Studio Build Tools installer
$installerUrl = "https://aka.ms/vs/17/release/vs_buildtools.exe"
$installerPath = "$env:TEMP\vs_buildtools.exe"

Write-Host "Downloading Visual Studio Build Tools installer..." -ForegroundColor Yellow
try {
    Invoke-WebRequest -Uri $installerUrl -OutFile $installerPath
    Write-Host "✓ Download completed" -ForegroundColor Green
} catch {
    Write-Host "✗ Download failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Install with C++ build tools workload
Write-Host "Installing Visual Studio Build Tools..." -ForegroundColor Yellow
Write-Host "This may take several minutes. Please wait..." -ForegroundColor Cyan

$installArgs = @(
    "--quiet",
    "--wait",
    "--norestart",
    "--nocache",
    "--installPath", "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools",
    "--add", "Microsoft.VisualStudio.Workload.VCTools",
    "--add", "Microsoft.VisualStudio.Component.Windows10SDK.19041"
)

try {
    Start-Process -FilePath $installerPath -ArgumentList $installArgs -Wait -NoNewWindow
    Write-Host "✓ Installation completed" -ForegroundColor Green
} catch {
    Write-Host "✗ Installation failed: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Clean up installer
Remove-Item $installerPath -Force

Write-Host "`nVisual Studio Build Tools installation completed!" -ForegroundColor Green
Write-Host "You may need to restart your terminal for the changes to take effect." -ForegroundColor Yellow
Write-Host "After restarting, try running: cargo build --release" -ForegroundColor Cyan 