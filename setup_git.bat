@echo off
echo ========================================
echo Git Setup for eSubpoena Tolls Tool
echo ========================================
echo.

REM Check if git is available
git --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Git is not installed or not in PATH
    echo.
    echo Please install Git first:
    echo 1. Run: winget install Git.Git
    echo 2. Restart this terminal
    echo 3. Run this script again
    echo.
    pause
    exit /b 1
)

echo Git is available: 
git --version
echo.

REM Configure Git identity
echo Configuring Git identity...
git config user.name "CAST"
git config user.email "bjdowning@fbi.gov"
echo ✓ Git identity configured
echo.

REM Initialize git repository
echo Initializing Git repository...
git init
if %errorlevel% neq 0 (
    echo ERROR: Failed to initialize Git repository
    pause
    exit /b 1
)

REM Add all files
echo Adding files to Git...
git add .
if %errorlevel% neq 0 (
    echo ERROR: Failed to add files
    pause
    exit /b 1
)

REM Create initial commit
echo Creating initial commit...
git commit -m "Initial commit: eSubpoena Tolls Tool Rust implementation"
if %errorlevel% neq 0 (
    echo ERROR: Failed to create commit
    echo Git identity may not be configured properly
    pause
    exit /b 1
)

REM Add remote
echo Adding GitHub remote...
git remote add origin https://github.com/casthuntsville1/esubpoena-tolls-tool.git
if %errorlevel% neq 0 (
    echo WARNING: Remote may already exist
)

REM Push to GitHub
echo Pushing to GitHub...
echo Note: You may be prompted to authenticate with GitHub
echo.
git push -u origin main
if %errorlevel% neq 0 (
    echo Trying with master branch...
    git push -u origin master
    if %errorlevel% neq 0 (
        echo ERROR: Failed to push to GitHub
        echo.
        echo This may be due to authentication. You have several options:
        echo.
        echo Option 1: Use GitHub CLI (recommended)
        echo 1. Install GitHub CLI: winget install GitHub.cli
        echo 2. Run: gh auth login
        echo 3. Try this script again
        echo.
        echo Option 2: Use Personal Access Token
        echo 1. Go to GitHub Settings > Developer settings > Personal access tokens
        echo 2. Generate a new token with repo permissions
        echo 3. Use the token as your password when prompted
        echo.
        echo Option 3: Manual upload
        echo 1. Go to: https://github.com/casthuntsville1/esubpoena-tolls-tool
        echo 2. Upload files manually using the web interface
        echo 3. Create a release manually
        echo.
        pause
        exit /b 1
    )
)

echo.
echo ========================================
echo SUCCESS! Code pushed to GitHub
echo ========================================
echo.
echo Next steps:
echo 1. Go to: https://github.com/casthuntsville1/esubpoena-tolls-tool
echo 2. Click "Releases" in the right sidebar
echo 3. Click "Create a new release"
echo 4. Create tag: v1.0.0
echo 5. Add release notes and publish
echo.
echo The GitHub Actions will automatically build and create the release!
echo.
pause 