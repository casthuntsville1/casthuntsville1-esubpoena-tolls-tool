# eSubpoena Tolls Tool - Rust Version

A high-performance Rust application for parsing telecommunication XML data and exporting to Excel with comprehensive analytics.

## Features

- **Fast XML Parsing**: Efficient parsing of telecommunication XML files using `quick-xml`
- **Modern GUI**: Native desktop application using `egui` framework
- **Comprehensive Analytics**: Detailed call analysis including:
  - Total, incoming, and outgoing calls
  - Call duration statistics
  - Most frequent numbers
  - Calls by day and hour
  - Target number analysis
  - Common contacts across multiple target numbers
- **Excel Export**: Professional Excel export with multiple worksheets
- **Multi-file Support**: Process multiple XML files simultaneously
- **Real-time Processing**: Background processing with progress updates

## Prerequisites

### Required Software
1. **Visual Studio Build Tools 2022** (for C++ compilation)
   - Download from: https://visualstudio.microsoft.com/downloads/#build-tools-for-visual-studio-2022
   - Install with "C++ build tools" workload

2. **Rust Toolchain**
   - Install via https://rustup.rs/
   - Or use: `winget install Rustlang.Rust.MSVC`

### Development Environment
- **Cursor IDE** with rust-analyzer extension
- **Windows 10/11** (tested)

## Installation

### Quick Setup
1. Run the setup script:
   ```bash
   setup_rust.bat
   ```

### Manual Setup
1. **Install Rust**:
   ```bash
   # Option 1: Using winget
   winget install Rustlang.Rust.MSVC
   
   # Option 2: Using rustup
   # Download from https://rustup.rs/
   ```

2. **Verify Installation**:
   ```bash
   rustc --version
   cargo --version
   ```

3. **Install rust-analyzer in Cursor**:
   - Open Cursor
   - Go to Extensions (Ctrl+Shift+X)
   - Search for "rust-analyzer"
   - Install the official extension
   - Restart Cursor

## Building and Running

### Development Build
```bash
cargo build
cargo run
```

### Release Build
```bash
cargo build --release
cargo run --release
```

### Standalone Executable
```bash
cargo build --release
# Executable will be in target/release/esubpoena-tolls-tool.exe
```

## Project Structure

```
src/
├── main.rs              # Application entry point
├── app.rs               # GUI application logic
├── data_models.rs       # Data structures and models
├── xml_parser.rs        # XML parsing functionality
├── analytics.rs         # Analytics calculations
└── excel_exporter.rs    # Excel export functionality
```

## Usage

### GUI Application
1. Run the application: `cargo run`
2. Use the file dialog to select XML files
3. View analytics in the different tabs:
   - **Overview**: Quick statistics and file selection
   - **Call Records**: Detailed call data table
   - **Analytics**: Comprehensive analytics dashboard
   - **Summary**: Text-based summary report
4. Export to Excel using the export button

### Command Line
```bash
# Run with specific XML file
cargo run -- path/to/file.xml

# Run in release mode for better performance
cargo run --release
```

## Data Format

The application expects XML files with the following structure:
```xml
<dataProduct>
  <xmlResult>
    <targetType>phone</targetType>
    <targetValue>1234567890</targetValue>
    <fromDate>2024-01-01</fromDate>
    <toDate>2024-01-31</toDate>
    <results>
      <messageDirection>incoming</messageDirection>
      <remoteNumber>9876543210</remoteNumber>
      <startTime>2024-01-01T10:00:00Z</startTime>
      <endTime>2024-01-01T10:05:00Z</endTime>
      <lengthOfCall>300</lengthOfCall>
    </results>
    <!-- More results... -->
  </xmlResult>
</dataProduct>
```

## Excel Export

The application exports to Excel with multiple worksheets:

1. **Call Records**: All call data with formatting
2. **Analytics**: Summary statistics and charts
3. **Summary Report**: Text-based analysis
4. **Common Contacts**: Contacts appearing across multiple target numbers

## Performance

The Rust version offers significant performance improvements over the Python version:
- **Faster parsing**: ~5-10x faster XML processing
- **Lower memory usage**: More efficient data structures
- **Native performance**: No interpreter overhead
- **Smaller executable**: Single binary distribution

## Troubleshooting

### Build Errors
1. **Linker errors**: Ensure Visual Studio Build Tools are installed
2. **Missing dependencies**: Run `cargo clean && cargo build`
3. **Rust version issues**: Update with `rustup update`

### Runtime Errors
1. **File not found**: Check file paths and permissions
2. **XML parsing errors**: Verify XML format matches expected structure
3. **Excel export errors**: Ensure write permissions to output directory

### Common Issues
- **rustc not found**: Restart terminal after Rust installation
- **Build tools missing**: Install Visual Studio Build Tools with C++ workload
- **Permission denied**: Run as administrator if needed

## Development

### Adding Features
1. Update data models in `src/data_models.rs`
2. Modify parsing logic in `src/xml_parser.rs`
3. Add analytics in `src/analytics.rs`
4. Update GUI in `src/app.rs`
5. Enhance Excel export in `src/excel_exporter.rs`

### Testing
```bash
# Run tests
cargo test

# Run with logging
RUST_LOG=info cargo run
```

## Dependencies

Key dependencies:
- `eframe/egui`: GUI framework
- `quick-xml`: XML parsing
- `xlsxwriter`: Excel export
- `chrono`: Date/time handling
- `serde`: Serialization
- `anyhow`: Error handling

## License

This project is for professional use in telecommunication data analysis.

## Support

For issues or questions:
1. Check the troubleshooting section
2. Review the code comments
3. Ensure all prerequisites are installed
4. Verify XML file format matches expected structure 