# eSubpoena Tolls Tool

A professional Python application for parsing telecommunication XML data and generating comprehensive Excel reports with analytics.

## Features

- **Drag & Drop Interface**: Easy-to-use GUI for processing XML files
- **XML Parsing**: Robust parsing of telecommunication data from various XML formats
- **Data Normalization**: Phone numbers are automatically normalized to 10-digit format
- **Comprehensive Analytics**: Detailed statistical analysis of call patterns
- **Excel Export**: Professional Excel output with multiple worksheets
- **Real-time Processing**: Background processing with progress indicators
- **Professional UI**: Modern, intuitive interface built with PyQt6

## Data Processing

The application processes telecommunication XML files containing:
- Call direction (Incoming/Outgoing)
- Remote phone numbers
- Start and end times
- Call duration
- Date and time information

### Data Normalization

Phone numbers are automatically cleaned and normalized:
- Removes all non-digit characters
- Converts to 10-digit format
- Handles various input formats (11-digit with country code, etc.)

## Excel Output

The exported Excel file contains three worksheets:

### 1. Call Records
- Complete call data with proper formatting
- Date/time columns formatted as Excel datetime
- Duration in both seconds and minutes
- Normalized phone numbers
- Day of week information

### 2. Analytics
- Summary statistics
- Most frequent phone numbers
- Calls by day analysis
- Calls by hour analysis

### 3. Summary Report
- Comprehensive text report
- All analytics in readable format
- Ready for documentation

## Installation

### Prerequisites
- Python 3.8 or higher
- Windows 10/11 (tested on Windows)

### Quick Start

1. **Run the application** (automatically installs dependencies):
   ```bash
   run_python_app.bat
   ```

### Manual Installation

1. **Install Python**: Download from https://www.python.org/downloads/
2. **Install dependencies**:
   ```bash
   pip install -r requirements.txt
   ```
3. **Run the application**:
   ```bash
   python esubpoena_tolls_tool.py
   ```

## Usage

1. **Launch the Application**: Run the executable or Python script
2. **Load XML Files**: 
   - Drag and drop XML files onto the drop zone
   - Or click the drop zone to browse for files
3. **View Results**: 
   - Overview tab shows quick statistics
   - Call Records tab displays all processed data
   - Analytics tab provides detailed analysis
   - Summary tab shows comprehensive report
4. **Export to Excel**: Click "Export to Excel" button to generate the report

## Supported XML Formats

The application can parse various XML formats including:
- Standard LDS-101 format
- Custom telecommunication XML formats
- XML files with nested call record structures

## Technical Details

### Architecture
- **GUI Framework**: PyQt6 for professional cross-platform interface
- **XML Parsing**: xml.etree.ElementTree for efficient XML processing
- **Excel Generation**: openpyxl for professional Excel output
- **Data Processing**: Custom analytics engine
- **Error Handling**: Comprehensive error handling with user feedback

### Performance
- Background processing for large files
- Memory-efficient data structures
- Optimized for handling thousands of call records

## File Structure

```
eSubpoena Tolls Tool/
├── esubpoena_tolls_tool.py    # Main application
├── requirements.txt           # Python dependencies
├── run_python_app.bat        # Quick start script
├── test_parser.py            # XML parsing test script
├── README.md                 # This file
└── LDS-101_item2_shipment1.xml  # Sample data file
```

## Error Handling

The application provides comprehensive error handling:
- Invalid XML files
- Missing or corrupted data
- File access issues
- Export failures

All errors are displayed to the user with helpful messages.

## Output Files

- **telecommunication_analysis.xlsx**: Main Excel report
- Generated in the same directory as the application
- Contains all processed data and analytics

## System Requirements

- **OS**: Windows 10/11
- **Python**: 3.8 or higher
- **Memory**: 4GB RAM minimum (8GB recommended)
- **Storage**: 100MB free space
- **Processor**: Any modern CPU

## Troubleshooting

### Common Issues

1. **Python Not Installed**
   - Download and install Python from https://www.python.org/downloads/
   - Make sure to check "Add Python to PATH" during installation

2. **Dependencies Not Installing**
   - Run `pip install --upgrade pip` first
   - Try running `run_python_app.bat` which handles installation automatically

3. **XML File Not Loading**
   - Ensure the file is a valid XML format
   - Check file permissions
   - Verify the file contains telecommunication data

4. **Export Fails**
   - Ensure sufficient disk space
   - Check write permissions in the output directory
   - Close any open Excel files with the same name

5. **Application Crashes**
   - Check system memory availability
   - Ensure XML file is not corrupted
   - Restart the application

### Performance Tips

- For very large files (>10,000 records), processing may take several seconds
- Close other applications to free up memory
- Use SSD storage for better performance

## Dependencies

The application uses the following Python packages:
- **PyQt6**: Professional GUI framework
- **pandas**: Data manipulation and analysis
- **openpyxl**: Excel file generation
- **lxml**: XML parsing
- **python-dateutil**: Date/time utilities

## License

This project is provided as-is for educational and professional use.

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review the error messages in the application
3. Ensure your XML file matches the expected format
4. Run `test_parser.py` to verify XML parsing works

## Version History

- **v1.0.0**: Python version with PyQt6 GUI
  - XML parsing and data processing
  - Excel export with analytics
  - Professional GUI interface
  - Drag and drop functionality
  - Background processing
  - Comprehensive error handling 