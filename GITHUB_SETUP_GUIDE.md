# GitHub Setup and Release Guide

This guide will help you push your code to GitHub and create automated releases.

## Prerequisites

### 1. Install Git (if not already installed)

**Option A: Using winget (recommended)**
```powershell
winget install Git.Git
```
*Note: You may need to run PowerShell as Administrator*

**Option B: Manual download**
1. Go to: https://git-scm.com/download/win
2. Download and install Git for Windows
3. Restart your terminal after installation

### 2. Verify Git Installation
```powershell
git --version
```

## Step 1: Initialize Git Repository

Open PowerShell in your project directory and run:

```powershell
# Initialize git repository
git init

# Configure your Git identity (replace with your details)
git config user.name "Your Name"
git config user.email "your.email@example.com"

# Add all files to git
git add .

# Create initial commit
git commit -m "Initial commit: eSubpoena Tolls Tool Rust implementation"
```

## Step 2: Connect to GitHub Repository

```powershell
# Add your GitHub repository as remote
git remote add origin https://github.com/casthuntsville1/esubpoena-tolls-tool.git

# Push to GitHub
git push -u origin main
```

*Note: If your default branch is `master` instead of `main`, use:*
```powershell
git push -u origin master
```

## Step 3: Create Your First Release

### Option A: Using GitHub Web Interface (Recommended)

1. **Go to your repository**: https://github.com/casthuntsville1/esubpoena-tolls-tool

2. **Create a tag**:
   - Click on "Releases" in the right sidebar
   - Click "Create a new release"
   - Click "Choose a tag" and type: `v1.0.0`
   - Click "Create new tag: v1.0.0 on publish"

3. **Fill in release details**:
   - **Release title**: `eSubpoena Tolls Tool v1.0.0`
   - **Description**:
   ```
   # eSubpoena Tolls Tool v1.0.0
   
   ## What's New
   - High-performance Rust implementation
   - Fast XML parsing and analytics
   - Professional Excel export
   - Modern GUI interface
   
   ## Installation
   1. Download the executable below
   2. Run `install.bat` to install to your desktop
   3. Or run the executable directly
   
   ## Features
   - Parse telecommunication XML data
   - Generate comprehensive analytics
   - Export to Excel with multiple worksheets
   - Multi-file support
   - Target number analysis
   - Common contacts detection
   
   ## System Requirements
   - Windows 10/11
   - No additional dependencies required
   
   ## Documentation
   See README_RUST.md for detailed usage instructions.
   ```

4. **Publish the release**: Click "Publish release"

### Option B: Using Command Line (if Git is working)

```powershell
# Create and push a tag
git tag -a "v1.0.0" -m "Release version 1.0.0"
git push origin "v1.0.0"
```

## Step 4: Monitor the Build Process

1. **Check GitHub Actions**:
   - Go to your repository
   - Click on "Actions" tab
   - You should see a workflow running for the tag you just created

2. **Wait for completion**:
   - The build process takes 5-10 minutes
   - You'll see two jobs: "build" and "create-release"

3. **Check the release**:
   - Once complete, go to "Releases" in your repository
   - You should see your v1.0.0 release with downloadable files

## Step 5: Download and Test

1. **Download the release**:
   - Go to your release page
   - Download the ZIP file containing the executable

2. **Test the installation**:
   - Extract the ZIP file
   - Run `install.bat` to install to desktop
   - Test the application

## Troubleshooting

### Git Issues
- **"git not recognized"**: Install Git and restart terminal
- **Authentication errors**: Use GitHub CLI or personal access token

### GitHub Actions Issues
- **Workflow not running**: Check that the `.github/workflows/build.yml` file exists
- **Build failures**: Check the Actions tab for error details
- **Permission issues**: Ensure the repository has Actions enabled

### Release Issues
- **No artifacts**: Check that the build job completed successfully
- **Missing files**: Verify all documentation files exist in your repository

## Future Releases

For future releases, simply:

1. **Update version in Cargo.toml**:
   ```toml
   [package]
   version = "1.0.1"  # Update this
   ```

2. **Commit and push changes**:
   ```powershell
   git add .
   git commit -m "Prepare for release v1.0.1"
   git push
   ```

3. **Create new tag**:
   ```powershell
   git tag -a "v1.0.1" -m "Release version 1.0.1"
   git push origin "v1.0.1"
   ```

4. **Or use GitHub web interface** to create a new release with tag `v1.0.1`

## Manual Alternative

If you prefer not to use Git, you can:

1. **Upload files manually** to your GitHub repository using the web interface
2. **Create releases manually** by uploading the built executable
3. **Use the installer script** locally: `.\installer\create_installer.ps1 -Version "1.0.0"`

## Support

If you encounter issues:
1. Check the GitHub Actions logs
2. Verify all files are properly committed
3. Ensure the workflow file is in the correct location
4. Check repository permissions and settings 