use crate::data_models::{CallRecord, DataProduct, Lds101Results, ProcessedCallRecord};
use anyhow::{Context, Result};
use log::{info, warn};
use quick_xml::de::from_str;
use std::fs;
use std::path::Path;

pub struct XmlParser;

impl XmlParser {
    pub fn parse_file(file_path: &Path) -> Result<Vec<ProcessedCallRecord>> {
        info!("Parsing XML file: {:?}", file_path);
        
        let content = fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {:?}", file_path))?;
        
        let source_file = file_path.file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("unknown")
            .to_string();
        
        Self::parse_content_with_source(&content, &source_file)
    }
    
    pub fn parse_content(content: &str) -> Result<Vec<ProcessedCallRecord>> {
        Self::parse_content_with_source(content, "unknown")
    }
    
    pub fn parse_content_with_source(content: &str, source_file: &str) -> Result<Vec<ProcessedCallRecord>> {
        // Try to parse as DataProduct first
        if let Ok(data_product) = from_str::<DataProduct>(content) {
            return Self::process_data_product_with_source(data_product, source_file);
        }
        
        // If that fails, try to parse as Lds101Results directly
        if let Ok(lds_results) = from_str::<Lds101Results>(content) {
            return Self::process_lds_results_with_source(lds_results, source_file);
        }
        
        // If both fail, try to extract results from the XML manually
        Self::parse_manual_with_source(content, source_file)
    }
    
    fn process_data_product(data_product: DataProduct) -> Result<Vec<ProcessedCallRecord>> {
        Self::process_lds_results_with_source(data_product.xml_result, "unknown")
    }
    
    fn process_data_product_with_source(data_product: DataProduct, source_file: &str) -> Result<Vec<ProcessedCallRecord>> {
        Self::process_lds_results_with_source(data_product.xml_result, source_file)
    }
    
    fn process_lds_results(lds_results: Lds101Results) -> Result<Vec<ProcessedCallRecord>> {
        Self::process_lds_results_with_source(lds_results, "unknown")
    }
    
    fn process_lds_results_with_source(lds_results: Lds101Results, source_file: &str) -> Result<Vec<ProcessedCallRecord>> {
        info!("Processing {} call records", lds_results.results.len());
        
        let mut processed_records = Vec::new();
        
        for (index, call_record) in lds_results.results.iter().enumerate() {
            match ProcessedCallRecord::from_call_record(call_record, &lds_results.target_value, source_file) {
                Ok(processed) => {
                    processed_records.push(processed);
                }
                Err(e) => {
                    warn!("Failed to process call record {}: {}", index, e);
                }
            }
        }
        
        info!("Successfully processed {} call records", processed_records.len());
        Ok(processed_records)
    }
    
    fn parse_manual(content: &str) -> Result<Vec<ProcessedCallRecord>> {
        Self::parse_manual_with_source(content, "unknown")
    }
    
    fn parse_manual_with_source(content: &str, source_file: &str) -> Result<Vec<ProcessedCallRecord>> {
        use quick_xml::events::Event;
        use quick_xml::Reader;
        
        let mut reader = Reader::from_str(content);
        reader.trim_text(true);
        
        let mut buf = Vec::new();
        let mut call_records = Vec::new();
        let mut current_record: Option<CallRecord> = None;
        let mut current_element = String::new();
        let mut target_value = String::new();
        
        loop {
            match reader.read_event_into(&mut buf)? {
                Event::Start(ref e) => {
                    current_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    
                    if current_element == "results" {
                        current_record = Some(CallRecord {
                            message_direction: String::new(),
                            remote_number: String::new(),
                            start_time: String::new(),
                            end_time: String::new(),
                            length_of_call: 0,
                        });
                    }
                }
                Event::Text(e) => {
                    let text = String::from_utf8_lossy(&e).to_string();
                    
                    if let Some(ref mut record) = current_record {
                        match current_element.as_str() {
                            "messageDirection" => record.message_direction = text,
                            "remoteNumber" => record.remote_number = text,
                            "startTime" => record.start_time = text,
                            "endTime" => record.end_time = text,
                            "lengthOfCall" => {
                                if let Ok(length) = text.parse::<u32>() {
                                    record.length_of_call = length;
                                }
                            }
                            _ => {}
                        }
                    } else if current_element == "targetValue" {
                        target_value = text;
                    }
                }
                Event::End(ref e) => {
                    let end_element = String::from_utf8_lossy(e.name().as_ref()).to_string();
                    
                    if end_element == "results" {
                        if let Some(record) = current_record.take() {
                            call_records.push(record);
                        }
                    }
                }
                Event::Eof => break,
                _ => {}
            }
            
            buf.clear();
        }
        
        info!("Manually parsed {} call records", call_records.len());
        
        let mut processed_records = Vec::new();
        for call_record in call_records {
            match ProcessedCallRecord::from_call_record(&call_record, &target_value, source_file) {
                Ok(processed) => processed_records.push(processed),
                Err(e) => warn!("Failed to process manually parsed record: {}", e),
            }
        }
        
        Ok(processed_records)
    }
} 