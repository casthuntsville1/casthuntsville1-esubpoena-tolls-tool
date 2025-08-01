use crate::data_models::{Analytics, ProcessedCallRecord};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use log::info;

pub struct AnalyticsEngine;

impl AnalyticsEngine {
    pub fn generate_analytics(records: &[ProcessedCallRecord]) -> Analytics {
        info!("Generating analytics for {} records", records.len());
        
        if records.is_empty() {
            return Analytics {
                total_calls: 0,
                incoming_calls: 0,
                outgoing_calls: 0,
                total_duration_minutes: 0.0,
                average_call_duration: 0.0,
                unique_numbers: 0,
                most_frequent_numbers: Vec::new(),
                calls_by_day: HashMap::new(),
                calls_by_hour: HashMap::new(),
                longest_call: None,
                shortest_call: None,
                target_numbers: std::collections::HashSet::new(),
                common_contacts: Vec::new(),
                files_processed: std::collections::HashSet::new(),
                date_range: (Utc::now(), Utc::now()),
            };
        }
        
        let total_calls = records.len();
        let incoming_calls = records.iter()
            .filter(|r| r.message_direction.eq_ignore_ascii_case("incoming"))
            .count();
        let outgoing_calls = total_calls - incoming_calls;
        
        let total_duration_minutes: f64 = records.iter()
            .map(|r| r.duration_minutes)
            .sum();
        
        let average_call_duration = if total_calls > 0 {
            total_duration_minutes / total_calls as f64
        } else {
            0.0
        };
        
        let unique_numbers = records.iter()
            .map(|r| &r.normalized_number)
            .collect::<std::collections::HashSet<_>>()
            .len();
        
        let mut number_counts: HashMap<String, usize> = HashMap::new();
        for record in records {
            *number_counts.entry(record.normalized_number.clone()).or_insert(0) += 1;
        }
        
        let mut most_frequent_numbers: Vec<(String, usize)> = number_counts.into_iter().collect();
        most_frequent_numbers.sort_by(|a, b| b.1.cmp(&a.1));
        most_frequent_numbers.truncate(10); // Top 10
        
        let mut calls_by_day: HashMap<String, usize> = HashMap::new();
        for record in records {
            *calls_by_day.entry(record.date.clone()).or_insert(0) += 1;
        }
        
        let mut calls_by_hour: HashMap<u32, usize> = HashMap::new();
        for record in records {
            let hour = record.start_time.hour();
            *calls_by_hour.entry(hour).or_insert(0) += 1;
        }
        
        let longest_call = records.iter()
            .max_by(|a, b| a.length_of_call.cmp(&b.length_of_call))
            .cloned();
        
        let shortest_call = records.iter()
            .filter(|r| r.length_of_call > 0) // Exclude 0-second calls
            .min_by(|a, b| a.length_of_call.cmp(&b.length_of_call))
            .cloned();
        
        let min_date = records.iter()
            .map(|r| r.start_time)
            .min()
            .unwrap_or_else(Utc::now);
        
        let max_date = records.iter()
            .map(|r| r.start_time)
            .max()
            .unwrap_or_else(Utc::now);
        
        let date_range = (min_date, max_date);
        
        // Collect target numbers and source files
        let target_numbers: std::collections::HashSet<String> = records.iter()
            .map(|r| r.target_number.clone())
            .filter(|t| !t.is_empty())
            .collect();
        
        let files_processed: std::collections::HashSet<String> = records.iter()
            .map(|r| r.source_file.clone())
            .filter(|f| !f.is_empty())
            .collect();
        
        // Find common contacts across target numbers
        let common_contacts = Self::find_common_contacts(records);
        
        Analytics {
            total_calls,
            incoming_calls,
            outgoing_calls,
            total_duration_minutes,
            average_call_duration,
            unique_numbers,
            most_frequent_numbers,
            calls_by_day,
            calls_by_hour,
            longest_call,
            shortest_call,
            target_numbers,
            common_contacts,
            files_processed,
            date_range,
        }
    }
    
    pub fn generate_summary_report(analytics: &Analytics, records: &[ProcessedCallRecord]) -> String {
        let mut report = String::new();
        
        report.push_str("=== TELECOMMUNICATION DATA ANALYSIS ===\n\n");
        
        report.push_str(&format!("Total Calls: {}\n", analytics.total_calls));
        report.push_str(&format!("Incoming Calls: {}\n", analytics.incoming_calls));
        report.push_str(&format!("Outgoing Calls: {}\n", analytics.outgoing_calls));
        report.push_str(&format!("Unique Phone Numbers: {}\n", analytics.unique_numbers));
        report.push_str(&format!("Target Numbers: {}\n", analytics.target_numbers.len()));
        report.push_str(&format!("Files Processed: {}\n", analytics.files_processed.len()));
        report.push_str(&format!("Total Duration: {:.2} minutes\n", analytics.total_duration_minutes));
        report.push_str(&format!("Average Call Duration: {:.2} minutes\n", analytics.average_call_duration));
        
        if let Some(longest) = &analytics.longest_call {
            report.push_str(&format!("Longest Call: {} seconds ({:.2} minutes) to {} on {}\n", 
                longest.length_of_call, longest.duration_minutes, longest.remote_number, longest.date));
        }
        
        if let Some(shortest) = &analytics.shortest_call {
            report.push_str(&format!("Shortest Call: {} seconds to {} on {}\n", 
                shortest.length_of_call, shortest.remote_number, shortest.date));
        }
        
        report.push_str(&format!("Date Range: {} to {}\n", 
            analytics.date_range.0.format("%Y-%m-%d"), 
            analytics.date_range.1.format("%Y-%m-%d")));
        
        report.push_str("\n=== MOST FREQUENT NUMBERS ===\n");
        for (i, (number, count)) in analytics.most_frequent_numbers.iter().enumerate() {
            report.push_str(&format!("{}. {} ({} calls)\n", i + 1, number, count));
        }
        
        report.push_str("\n=== TARGET NUMBERS ===\n");
        for target_num in &analytics.target_numbers {
            let target_records: Vec<_> = records.iter()
                .filter(|r| r.target_number == *target_num)
                .collect();
            report.push_str(&format!("• {}: {} calls\n", target_num, target_records.len()));
        }
        
        if !analytics.common_contacts.is_empty() {
            report.push_str("\n=== COMMON CONTACTS ACROSS TARGET NUMBERS ===\n");
            for contact in &analytics.common_contacts {
                let target_nums = contact.target_numbers.join(", ");
                report.push_str(&format!("• {}: appears in {} target numbers ({})\n", 
                    contact.number, contact.count, target_nums));
            }
        }
        
        report.push_str("\n=== CALLS BY DAY ===\n");
        let mut sorted_days: Vec<_> = analytics.calls_by_day.iter().collect();
        sorted_days.sort_by(|a, b| a.0.cmp(b.0));
        for (day, count) in sorted_days {
            report.push_str(&format!("{}: {} calls\n", day, count));
        }
        
        report.push_str("\n=== CALLS BY HOUR ===\n");
        for hour in 0..24 {
            if let Some(count) = analytics.calls_by_hour.get(&hour) {
                report.push_str(&format!("{:02}:00: {} calls\n", hour, count));
            }
        }
        
        report
    }
    
    fn find_common_contacts(records: &[ProcessedCallRecord]) -> Vec<crate::data_models::CommonContact> {
        use std::collections::HashMap;
        
        // Group records by target number
        let mut target_groups: HashMap<String, std::collections::HashSet<String>> = HashMap::new();
        for record in records {
            if !record.target_number.is_empty() {
                target_groups.entry(record.target_number.clone())
                    .or_insert_with(std::collections::HashSet::new)
                    .insert(record.normalized_number.clone());
            }
        }
        
        // Find numbers that appear in multiple target groups
        if target_groups.len() <= 1 {
            return Vec::new();
        }
        
        let mut all_numbers = std::collections::HashSet::new();
        for numbers in target_groups.values() {
            all_numbers.extend(numbers.iter().cloned());
        }
        
        let mut common_contacts = Vec::new();
        for number in all_numbers {
            let target_numbers_with_contact: Vec<String> = target_groups.iter()
                .filter(|(_, numbers)| numbers.contains(&number))
                .map(|(target, _)| target.clone())
                .collect();
            
            if target_numbers_with_contact.len() > 1 {
                common_contacts.push(crate::data_models::CommonContact {
                    number,
                    target_numbers: target_numbers_with_contact,
                    count: target_numbers_with_contact.len(),
                });
            }
        }
        
        // Sort by number of target numbers they appear in
        common_contacts.sort_by(|a, b| b.count.cmp(&a.count));
        common_contacts
    }
} 