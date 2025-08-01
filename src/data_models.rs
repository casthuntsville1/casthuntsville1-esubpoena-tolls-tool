use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProduct {
    #[serde(rename = "xmlResult")]
    pub xml_result: Lds101Results,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Lds101Results {
    #[serde(rename = "targetType")]
    pub target_type: String,
    #[serde(rename = "targetValue")]
    pub target_value: String,
    #[serde(rename = "fromDate")]
    pub from_date: String,
    #[serde(rename = "toDate")]
    pub to_date: String,
    pub results: Vec<CallRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallRecord {
    #[serde(rename = "messageDirection")]
    pub message_direction: String,
    #[serde(rename = "remoteNumber")]
    pub remote_number: String,
    #[serde(rename = "startTime")]
    pub start_time: String,
    #[serde(rename = "endTime")]
    pub end_time: String,
    #[serde(rename = "lengthOfCall")]
    pub length_of_call: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessedCallRecord {
    pub message_direction: String,
    pub remote_number: String,
    pub normalized_number: String,
    pub target_number: String,
    pub source_file: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub length_of_call: u32,
    pub duration_minutes: f64,
    pub date: String,
    pub time: String,
    pub date_time: String,
    pub day_of_week: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Analytics {
    pub total_calls: usize,
    pub incoming_calls: usize,
    pub outgoing_calls: usize,
    pub total_duration_minutes: f64,
    pub average_call_duration: f64,
    pub unique_numbers: usize,
    pub most_frequent_numbers: Vec<(String, usize)>,
    pub calls_by_day: HashMap<String, usize>,
    pub calls_by_hour: HashMap<u32, usize>,
    pub longest_call: Option<ProcessedCallRecord>,
    pub shortest_call: Option<ProcessedCallRecord>,
    pub target_numbers: std::collections::HashSet<String>,
    pub common_contacts: Vec<CommonContact>,
    pub files_processed: std::collections::HashSet<String>,
    pub date_range: (DateTime<Utc>, DateTime<Utc>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonContact {
    pub number: String,
    pub target_numbers: Vec<String>,
    pub count: usize,
}

impl ProcessedCallRecord {
    pub fn from_call_record(call: &CallRecord, target_number: &str, source_file: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let start_time = DateTime::parse_from_rfc3339(&call.start_time)?.with_timezone(&Utc);
        let end_time = DateTime::parse_from_rfc3339(&call.end_time)?.with_timezone(&Utc);
        
        let normalized_number = normalize_phone_number(&call.remote_number);
        let duration_minutes = call.length_of_call as f64 / 60.0;
        
        Ok(Self {
            message_direction: call.message_direction.clone(),
            remote_number: call.remote_number.clone(),
            normalized_number,
            target_number: target_number.to_string(),
            source_file: source_file.to_string(),
            start_time,
            end_time,
            length_of_call: call.length_of_call,
            duration_minutes,
            date: start_time.format("%Y-%m-%d").to_string(),
            time: start_time.format("%H:%M:%S").to_string(),
            date_time: start_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            day_of_week: start_time.format("%A").to_string(),
        })
    }
}

pub fn normalize_phone_number(number: &str) -> String {
    use regex::Regex;
    use lazy_static::lazy_static;
    
    lazy_static! {
        static ref PHONE_REGEX: Regex = Regex::new(r"[^\d]").unwrap();
    }
    
    let digits_only = PHONE_REGEX.replace_all(number, "");
    
    // If it's a 10-digit number, return as is
    if digits_only.len() == 10 {
        return digits_only.to_string();
    }
    
    // If it's 11 digits and starts with 1, remove the 1
    if digits_only.len() == 11 && digits_only.starts_with('1') {
        return digits_only[1..].to_string();
    }
    
    // If it's longer than 10 digits, take the last 10
    if digits_only.len() > 10 {
        return digits_only[digits_only.len() - 10..].to_string();
    }
    
    // Otherwise, pad with zeros to make it 10 digits
    format!("{:0>10}", digits_only)
} 