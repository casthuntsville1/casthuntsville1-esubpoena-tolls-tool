# Quick Start Guide - eSubpoena Tolls Tool

## 🚀 Get Started in 3 Steps

### Step 1: Install Rust
1. Visit https://rustup.rs/
2. Download and run the installer
3. Restart your terminal/command prompt

### Step 2: Build the Application
```bash
cargo build --release
```

### Step 3: Run the Application
```bash
cargo run --release
```

## 📁 What You Get

The application will create a professional GUI with:
- **Drag & Drop Interface** for XML files
- **Real-time Analytics** dashboard
- **Excel Export** with formatted data
- **Comprehensive Reports** with statistics

## 📊 Sample Data Processing

The application successfully processes your `LDS-101_item2_shipment1.xml` file:
- **3,576 call records** found
- **Phone numbers normalized** to 10-digit format
- **Date/time formatting** for Excel compatibility
- **Duration calculations** in seconds and minutes

## 📈 Excel Output

The exported Excel file contains:
1. **Call Records** - All processed data with proper formatting
2. **Analytics** - Statistical analysis and charts
3. **Summary Report** - Comprehensive text report

## 🛠️ Troubleshooting

### If Rust is not installed:
- Run `install_rust.bat` for installation instructions
- Or visit https://rustup.rs/ directly

### If build fails:
- Ensure Rust is properly installed
- Restart terminal after Rust installation
- Run `build_and_run.bat` for automated build and run

### If XML file doesn't load:
- Verify the file is valid XML
- Check file permissions
- Ensure the file contains telecommunication data

## 📞 Support

- Check the main `README.md` for detailed documentation
- Run `test_parser.py` to verify XML parsing works
- Review error messages in the application

## 🎯 Expected Results

After processing your XML file, you should see:
- Total calls: ~3,576
- Incoming/outgoing call breakdown
- Most frequent phone numbers
- Call patterns by day and hour
- Professional Excel export with all data

---

**Ready to analyze your telecommunication data!** 📞📊 