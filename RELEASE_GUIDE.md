# GitHub Release Guide for eSubpoena Tolls Tool

This guide will help you create GitHub releases with installable packages for your users.

## Prerequisites

1. **GitHub Repository**: Your code must be pushed to a GitHub repository
2. **GitHub CLI** (optional): For automated releases
3. **Rust Toolchain**: For building the application

## Step 1: Set Up GitHub Repository

### Create a new repository on GitHub:
1. Go to https://github.com/new
2. Name it `esubpoena-tolls-tool`
3. Make it public or private as needed
4. Don't initialize with README (we already have one)

### Push your code to GitHub:
```bash
# Initialize git repository (if not already done)
git init

# Add all files
git add .

# Create initial commit
git commit -m "Initial commit: eSubpoena Tolls Tool Rust implementation"

# Add GitHub remote (replace with your username)
git remote add origin https://github.com/YOUR_USERNAME/esubpoena-tolls-tool.git

# Push to GitHub
git push -u origin main
```

## Step 2: Test Local Build

Before creating a release, test that everything builds correctly:

```bash
# Build the project
cargo build --release

# Test the executable
cargo run --release
```

## Step 3: Create a Release

### Option A: Automated Release (Recommended)

1. **Install GitHub CLI** (if not already installed):
   ```bash
   winget install GitHub.cli
   ```

2. **Authenticate with GitHub**:
   ```bash
   gh auth login
   ```

3. **Update the repository name** in `scripts/release.ps1`:
   ```powershell
   [string]$GitHubRepo = "YOUR_USERNAME/esubpoena-tolls-tool"
   ```

4. **Create a release**:
   ```powershell
   .\scripts\release.ps1 -Version "1.0.0"
   ```

### Option B: Manual Release

1. **Build and create installer**:
   ```powershell
   cargo build --release
   .\installer\create_installer.ps1 -Version "1.0.0"
   ```

2. **Create git tag**:
   ```bash
   git tag -a "v1.0.0" -m "Release version 1.0.0"
   git push origin "v1.0.0"
   ```

3. **Create GitHub release**:
   - Go to your repository on GitHub
   - Click "Releases" in the right sidebar
   - Click "Create a new release"
   - Select the tag you just created
   - Upload the ZIP file from `installer/output/`
   - Add release notes
   - Publish the release

## Step 4: GitHub Actions (Automatic)

The GitHub Actions workflow will automatically:
- Build the project when you push a tag
- Create a release with the compiled executable
- Upload installable packages

To trigger this:
```bash
git tag -a "v1.0.1" -m "Release version 1.0.1"
git push origin "v1.0.1"
```

## Release Package Contents

Each release includes:
- `esubpoena-tolls-tool.exe` - The main executable
- `install.bat` - Installation script
- `uninstall.bat` - Uninstallation script
- `README_INSTALLER.md` - Installation instructions
- `README_RUST.md` - Full documentation
- `RUST_SETUP_GUIDE.md` - Development setup guide
- `CONVERSION_SUMMARY.md` - Conversion details

## User Installation Process

Users can install the application by:

1. **Download** the ZIP file from the GitHub release
2. **Extract** the contents to a folder
3. **Run** `install.bat` to install to desktop
4. **Use** the desktop shortcut to launch the application

## Version Management

### Semantic Versioning
Use semantic versioning for releases:
- `MAJOR.MINOR.PATCH`
- Example: `1.0.0`, `1.1.0`, `1.1.1`

### Update Cargo.toml
Before each release, update the version in `Cargo.toml`:
```toml
[package]
name = "esubpoena-tolls-tool"
version = "1.0.0"  # Update this
```

## Troubleshooting

### Build Issues
- Ensure Rust is properly installed: `rustc --version`
- Check Visual Studio Build Tools are installed
- Run `cargo clean && cargo build --release`

### GitHub Issues
- Ensure you have write access to the repository
- Check GitHub CLI authentication: `gh auth status`
- Verify the repository name in release scripts

### Release Issues
- Check that the tag exists: `git tag -l`
- Ensure the tag is pushed: `git push origin --tags`
- Verify GitHub Actions workflow is enabled

## Best Practices

1. **Test thoroughly** before releasing
2. **Use meaningful version numbers**
3. **Write clear release notes**
4. **Include changelog information**
5. **Test the installer on a clean system**

## Example Release Workflow

```bash
# 1. Update version in Cargo.toml
# 2. Test the build
cargo build --release
cargo test

# 3. Commit changes
git add .
git commit -m "Prepare for release v1.0.0"

# 4. Create and push tag
git tag -a "v1.0.0" -m "Release version 1.0.0"
git push origin "v1.0.0"

# 5. Create release (automated via GitHub Actions)
# Or manually create release on GitHub
```

## Support

If you encounter issues:
1. Check the GitHub Actions logs
2. Verify all prerequisites are installed
3. Test the build process locally
4. Check GitHub repository permissions

The automated release process will make it easy for users to download and install your application! 