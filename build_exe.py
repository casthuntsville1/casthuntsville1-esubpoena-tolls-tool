#!/usr/bin/env python3
"""
Build script for eSubpoena Tolls Tool executable.
This script creates the icon and compiles the application using PyInstaller.
"""

import os
import subprocess
import sys

def install_requirements():
    """Install required packages for building."""
    print("📦 Installing required packages...")
    
    packages = [
        "pyinstaller",
        "pillow",
        "openpyxl"
    ]
    
    for package in packages:
        try:
            subprocess.check_call([sys.executable, "-m", "pip", "install", package])
            print(f"✅ Installed {package}")
        except subprocess.CalledProcessError:
            print(f"❌ Failed to install {package}")
            return False
    
    return True

def create_icon():
    """Create the phone icon."""
    print("🎨 Creating phone icon...")
    
    try:
        # First install Pillow if not already installed
        subprocess.check_call([sys.executable, "-m", "pip", "install", "pillow"])
        
        # Run the icon creation script
        subprocess.check_call([sys.executable, "create_phone_icon.py"])
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Failed to create icon: {e}")
        return False

def build_executable():
    """Build the executable using PyInstaller."""
    print("🔨 Building executable...")
    
    # PyInstaller command
    cmd = [
        "pyinstaller",
        "--onefile",                    # Single executable file
        "--windowed",                   # No console window
        "--icon=phone_icon.ico",        # Use our phone icon
        "--name=eSubpoena_Tolls_Tool",  # Executable name
        "--add-data=phone_icon.ico;.",  # Include icon in executable
        "esubpoena_tolls_tool_simple.py"
    ]
    
    try:
        subprocess.check_call(cmd)
        print("✅ Executable built successfully!")
        print(f"📁 Location: dist/eSubpoena_Tolls_Tool.exe")
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ Build failed: {e}")
        return False

def main():
    """Main build process."""
    print("🚀 Starting eSubpoena Tolls Tool build process...")
    print("=" * 50)
    
    # Step 1: Install requirements
    if not install_requirements():
        print("❌ Failed to install requirements")
        return
    
    # Step 2: Create icon
    if not create_icon():
        print("❌ Failed to create icon")
        return
    
    # Step 3: Build executable
    if not build_executable():
        print("❌ Failed to build executable")
        return
    
    print("=" * 50)
    print("🎉 Build completed successfully!")
    print("📱 Your executable is ready: dist/eSubpoena_Tolls_Tool.exe")
    print("📋 Features:")
    print("   ✅ Single executable file")
    print("   ✅ No console window")
    print("   ✅ Cell phone icon")
    print("   ✅ Professional appearance")

if __name__ == "__main__":
    main() 