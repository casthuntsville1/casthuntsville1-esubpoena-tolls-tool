use crate::data_models::{Analytics, ProcessedCallRecord};
use anyhow::{Context, Result};
use log::info;
use std::path::Path;
use xlsxwriter::{Format, Workbook, Worksheet};

pub struct ExcelExporter;

impl ExcelExporter {
    pub fn export_data(
        records: &[ProcessedCallRecord],
        analytics: &Analytics,
        output_path: &Path,
    ) -> Result<()> {
        info!("Exporting data to Excel: {:?}", output_path);
        
        let workbook = Workbook::new(output_path.to_str().unwrap())
            .with_context(|| format!("Failed to create workbook at {:?}", output_path))?;
        
        // Create formats
        let header_format = workbook.add_format()
            .set_bold()
            .set_bg_color(xlsxwriter::FormatColor::Gray)
            .set_border()
            .set_align(xlsxwriter::FormatAlignment::Center);
        
        let date_format = workbook.add_format()
            .set_num_format("yyyy-mm-dd hh:mm:ss")
            .set_border();
        
        let number_format = workbook.add_format()
            .set_num_format("0")
            .set_border();
        
        let duration_format = workbook.add_format()
            .set_num_format("0.00")
            .set_border();
        
        let text_format = workbook.add_format()
            .set_border();
        
        // Export call records
        Self::export_call_records(&workbook, records, &header_format, &date_format, &number_format, &duration_format, &text_format)?;
        
        // Export analytics
        Self::export_analytics(&workbook, analytics, &header_format, &text_format, &number_format)?;
        
        // Export summary report
        Self::export_summary_report(&workbook, analytics, records, &header_format, &text_format)?;
        
        // Export common contacts
        Self::export_common_contacts(&workbook, analytics, &header_format, &text_format)?;
        
        workbook.close()
            .with_context(|| "Failed to close workbook")?;
        
        info!("Successfully exported data to Excel");
        Ok(())
    }
    
    fn export_call_records(
        workbook: &Workbook,
        records: &[ProcessedCallRecord],
        header_format: &Format,
        date_format: &Format,
        number_format: &Format,
        duration_format: &Format,
        text_format: &Format,
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet(Some("Call Records"))?;
        
        // Set column widths
        worksheet.set_column(0, 0, 15.0, None)?; // Direction
        worksheet.set_column(1, 1, 15.0, None)?; // Target Number
        worksheet.set_column(2, 2, 15.0, None)?; // Remote Number
        worksheet.set_column(3, 3, 15.0, None)?; // Normalized Number
        worksheet.set_column(4, 4, 20.0, None)?; // Date & Time
        worksheet.set_column(5, 5, 20.0, None)?; // End Time
        worksheet.set_column(6, 6, 12.0, None)?; // Duration (seconds)
        worksheet.set_column(7, 7, 15.0, None)?; // Duration (minutes)
        worksheet.set_column(8, 8, 12.0, None)?; // Day of Week
        worksheet.set_column(9, 9, 15.0, None)?; // Source File
        
        // Write headers
        let headers = [
            "Direction", "Target Number", "Remote Number", "Normalized Number", "Date & Time", "End Time",
            "Duration (sec)", "Duration (min)", "Day of Week", "Source File"
        ];
        
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string(0, col as u16, header, Some(header_format))?;
        }
        
        // Write data
        for (row, record) in records.iter().enumerate() {
            let row_num = (row + 1) as u32;
            
            worksheet.write_string(row_num, 0, &record.message_direction, Some(text_format))?;
            worksheet.write_string(row_num, 1, &record.target_number, Some(text_format))?;
            worksheet.write_string(row_num, 2, &record.remote_number, Some(text_format))?;
            worksheet.write_string(row_num, 3, &record.normalized_number, Some(text_format))?;
            worksheet.write_string(row_num, 4, &record.date_time, Some(text_format))?;
            worksheet.write_datetime(row_num, 5, &record.end_time, Some(date_format))?;
            worksheet.write_number(row_num, 6, record.length_of_call as f64, Some(number_format))?;
            worksheet.write_number(row_num, 7, record.duration_minutes, Some(duration_format))?;
            worksheet.write_string(row_num, 8, &record.day_of_week, Some(text_format))?;
            worksheet.write_string(row_num, 9, &record.source_file, Some(text_format))?;
        }
        
        Ok(())
    }
    
    fn export_analytics(
        workbook: &Workbook,
        analytics: &Analytics,
        header_format: &Format,
        text_format: &Format,
        number_format: &Format,
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet(Some("Analytics"))?;
        
        // Set column widths
        worksheet.set_column(0, 0, 25.0, None)?;
        worksheet.set_column(1, 1, 15.0, None)?;
        
        // Summary statistics
        let summary_data = [
            ("Total Calls", analytics.total_calls.to_string()),
            ("Incoming Calls", analytics.incoming_calls.to_string()),
            ("Outgoing Calls", analytics.outgoing_calls.to_string()),
            ("Unique Phone Numbers", analytics.unique_numbers.to_string()),
            ("Total Duration (minutes)", format!("{:.2}", analytics.total_duration_minutes)),
            ("Average Call Duration (minutes)", format!("{:.2}", analytics.average_call_duration)),
        ];
        
        worksheet.write_string(0, 0, "Metric", Some(header_format))?;
        worksheet.write_string(0, 1, "Value", Some(header_format))?;
        
        for (row, (metric, value)) in summary_data.iter().enumerate() {
            let row_num = (row + 1) as u32;
            worksheet.write_string(row_num, 0, metric, Some(text_format))?;
            worksheet.write_string(row_num, 1, value, Some(text_format))?;
        }
        
        // Most frequent numbers
        let start_row = (summary_data.len() + 3) as u32;
        worksheet.write_string(start_row, 0, "Most Frequent Numbers", Some(header_format))?;
        worksheet.write_string(start_row, 1, "Call Count", Some(header_format))?;
        
        for (i, (number, count)) in analytics.most_frequent_numbers.iter().enumerate() {
            let row_num = start_row + 1 + i as u32;
            worksheet.write_string(row_num, 0, number, Some(text_format))?;
            worksheet.write_number(row_num, 1, *count as f64, Some(number_format))?;
        }
        
        // Calls by day
        let day_start_row = start_row + analytics.most_frequent_numbers.len() as u32 + 3;
        worksheet.write_string(day_start_row, 0, "Calls by Day", Some(header_format))?;
        worksheet.write_string(day_start_row, 1, "Call Count", Some(header_format))?;
        
        let mut sorted_days: Vec<_> = analytics.calls_by_day.iter().collect();
        sorted_days.sort_by(|a, b| a.0.cmp(b.0));
        
        for (i, (day, count)) in sorted_days.iter().enumerate() {
            let row_num = day_start_row + 1 + i as u32;
            worksheet.write_string(row_num, 0, day, Some(text_format))?;
            worksheet.write_number(row_num, 1, **count as f64, Some(number_format))?;
        }
        
        // Calls by hour
        let hour_start_row = day_start_row + sorted_days.len() as u32 + 3;
        worksheet.write_string(hour_start_row, 0, "Calls by Hour", Some(header_format))?;
        worksheet.write_string(hour_start_row, 1, "Call Count", Some(header_format))?;
        
        for hour in 0..24 {
            if let Some(count) = analytics.calls_by_hour.get(&hour) {
                let row_num = hour_start_row + 1 + hour as u32;
                worksheet.write_string(row_num, 0, &format!("{:02}:00", hour), Some(text_format))?;
                worksheet.write_number(row_num, 1, *count as f64, Some(number_format))?;
            }
        }
        
        Ok(())
    }
    
    fn export_summary_report(
        workbook: &Workbook,
        analytics: &Analytics,
        records: &[ProcessedCallRecord],
        header_format: &Format,
        text_format: &Format,
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet(Some("Summary Report"))?;
        
        // Set column width
        worksheet.set_column(0, 0, 80.0, None)?;
        
        let report = crate::analytics::AnalyticsEngine::generate_summary_report(analytics, records);
        let lines: Vec<&str> = report.lines().collect();
        
        for (row, line) in lines.iter().enumerate() {
            let row_num = row as u32;
            if line.starts_with("===") {
                worksheet.write_string(row_num, 0, line, Some(header_format))?;
            } else {
                worksheet.write_string(row_num, 0, line, Some(text_format))?;
            }
        }
        
        Ok(())
    }
    
    fn export_common_contacts(
        workbook: &Workbook,
        analytics: &Analytics,
        header_format: &Format,
        text_format: &Format,
    ) -> Result<()> {
        let worksheet = workbook.add_worksheet(Some("Common Contacts"))?;
        
        // Set column widths
        worksheet.set_column(0, 0, 15.0, None)?; // Phone Number
        worksheet.set_column(1, 1, 40.0, None)?; // Target Numbers
        worksheet.set_column(2, 2, 10.0, None)?; // Count
        
        // Write headers
        let headers = ["Phone Number", "Target Numbers", "Count"];
        for (col, header) in headers.iter().enumerate() {
            worksheet.write_string(0, col as u16, header, Some(header_format))?;
        }
        
        if analytics.common_contacts.is_empty() {
            worksheet.write_string(1, 0, "No common contacts found across target numbers", Some(text_format))?;
            return Ok(());
        }
        
        // Write data
        for (row, contact) in analytics.common_contacts.iter().enumerate() {
            let row_num = (row + 1) as u32;
            let target_nums = contact.target_numbers.join(", ");
            
            worksheet.write_string(row_num, 0, &contact.number, Some(text_format))?;
            worksheet.write_string(row_num, 1, &target_nums, Some(text_format))?;
            worksheet.write_number(row_num, 2, contact.count as f64, Some(text_format))?;
        }
        
        Ok(())
    }
} 