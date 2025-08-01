use crate::analytics::AnalyticsEngine;
use crate::data_models::{Analytics, ProcessedCallRecord};
use crate::excel_exporter::ExcelExporter;
use crate::xml_parser::XmlParser;
use eframe::egui;
use log::{error, info};
use std::path::PathBuf;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub struct EsubpoenaApp {
    // Data
    call_records: Vec<ProcessedCallRecord>,
    analytics: Option<Analytics>,
    
    // UI State
    drag_state: DragState,
    processing_state: ProcessingState,
    selected_tab: Tab,
    
    // Messages
    messages: Vec<Message>,
    
    // Background processing
    processing_sender: Option<mpsc::Sender<ProcessingMessage>>,
    processing_receiver: Option<mpsc::Receiver<ProcessingMessage>>,
}

#[derive(Debug, Clone)]
enum DragState {
    None,
    Hovering,
    Dropping,
}

#[derive(Debug, Clone)]
enum ProcessingState {
    Idle,
    Processing,
    Completed,
    Error(String),
}

#[derive(Debug, Clone)]
enum Tab {
    Overview,
    CallRecords,
    Analytics,
    Summary,
}

#[derive(Debug, Clone)]
enum Message {
    Info(String),
    Success(String),
    Warning(String),
    Error(String),
}

#[derive(Debug)]
enum ProcessingMessage {
    Progress(String),
    Completed(Vec<ProcessedCallRecord>),
    Error(String),
}

impl EsubpoenaApp {
    pub fn new() -> Self {
        Self {
            call_records: Vec::new(),
            analytics: None,
            drag_state: DragState::None,
            processing_state: ProcessingState::Idle,
            selected_tab: Tab::Overview,
            messages: Vec::new(),
            processing_sender: None,
            processing_receiver: None,
        }
    }
    
    fn add_message(&mut self, message: Message) {
        self.messages.push(message);
        if self.messages.len() > 10 {
            self.messages.remove(0);
        }
    }
    
    fn process_file(&mut self, file_path: PathBuf) {
        info!("Processing file: {:?}", file_path);
        self.processing_state = ProcessingState::Processing;
        self.add_message(Message::Info(format!("Processing file: {}", file_path.display())));
        
        let (sender, receiver) = mpsc::channel();
        self.processing_sender = Some(sender);
        self.processing_receiver = Some(receiver);
        
        thread::spawn(move || {
            match XmlParser::parse_file(&file_path) {
                Ok(records) => {
                    let _ = sender.send(ProcessingMessage::Completed(records));
                }
                Err(e) => {
                    let _ = sender.send(ProcessingMessage::Error(e.to_string()));
                }
            }
        });
    }
    
    fn export_to_excel(&mut self) {
        if self.call_records.is_empty() {
            self.add_message(Message::Warning("No data to export".to_string()));
            return;
        }
        
        if let Some(analytics) = &self.analytics {
            let output_path = PathBuf::from("telecommunication_analysis.xlsx");
            
            match ExcelExporter::export_data(&self.call_records, analytics, &output_path) {
                Ok(_) => {
                    self.add_message(Message::Success(format!(
                        "Successfully exported to: {}",
                        output_path.display()
                    )));
                }
                Err(e) => {
                    self.add_message(Message::Error(format!("Export failed: {}", e)));
                }
            }
        }
    }
}

impl eframe::App for EsubpoenaApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Check for background processing messages
        if let Some(receiver) = &self.processing_receiver {
            if let Ok(message) = receiver.try_recv() {
                match message {
                    ProcessingMessage::Progress(msg) => {
                        self.add_message(Message::Info(msg));
                    }
                    ProcessingMessage::Completed(records) => {
                        self.call_records = records;
                        self.analytics = Some(AnalyticsEngine::generate_analytics(&self.call_records));
                        self.processing_state = ProcessingState::Completed;
                        self.add_message(Message::Success(format!(
                            "Successfully processed {} call records",
                            self.call_records.len()
                        )));
                    }
                    ProcessingMessage::Error(error_msg) => {
                        self.processing_state = ProcessingState::Error(error_msg.clone());
                        self.add_message(Message::Error(error_msg));
                    }
                }
            }
        }
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_header(ui);
            ui.add_space(10.0);
            
            match self.selected_tab {
                Tab::Overview => self.render_overview(ui),
                Tab::CallRecords => self.render_call_records(ui),
                Tab::Analytics => self.render_analytics(ui),
                Tab::Summary => self.render_summary(ui),
            }
        });
    }
}

impl EsubpoenaApp {
    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("📞 eSubpoena Tolls Tool");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Export to Excel").clicked() {
                    self.export_to_excel();
                }
            });
        });
        
        ui.separator();
        
        // Tab bar
        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.selected_tab, Tab::Overview, "Overview");
            ui.selectable_value(&mut self.selected_tab, Tab::CallRecords, "Call Records");
            ui.selectable_value(&mut self.selected_tab, Tab::Analytics, "Analytics");
            ui.selectable_value(&mut self.selected_tab, Tab::Summary, "Summary");
        });
    }
    
    fn render_overview(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("Welcome to eSubpoena Tolls Tool");
            ui.label("This tool processes telecommunication XML data and provides comprehensive analytics.");
            
            ui.add_space(20.0);
            
            // Drag and drop area
            let (response, painter) = ui.allocate_painter(
                egui::vec2(ui.available_width(), 200.0),
                egui::Sense::drag(),
            );
            
            let rect = response.rect;
            let is_hovered = response.hovered();
            let is_dragged = response.dragged();
            
            // Update drag state
            self.drag_state = if is_dragged {
                DragState::Dropping
            } else if is_hovered {
                DragState::Hovering
            } else {
                DragState::None
            };
            
            // Draw drop zone
            let color = match self.drag_state {
                DragState::None => egui::Color32::from_gray(50),
                DragState::Hovering => egui::Color32::from_gray(70),
                DragState::Dropping => egui::Color32::from_rgb(100, 150, 255),
            };
            
            painter.rect_filled(rect, 5.0, color);
            painter.rect_stroke(rect, 2.0, (2.0, egui::Color32::from_gray(100)));
            
            // Draw text
            let text = match self.drag_state {
                DragState::None => "📁 Drag and drop XML files here\nor click to browse",
                DragState::Hovering => "📁 Drop XML files here",
                DragState::Dropping => "📁 Processing...",
            };
            
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(18.0),
                egui::Color32::WHITE,
            );
            
            // Handle file drops
            if !response.dropped_files().is_empty() {
                for dropped_file in response.dropped_files() {
                    if let Some(path) = &dropped_file.path {
                        if path.extension().map_or(false, |ext| ext == "xml") {
                            self.process_file(path.clone());
                        } else {
                            self.add_message(Message::Warning("Please drop XML files only".to_string()));
                        }
                    }
                }
            }
            
            // Handle click to browse
            if response.clicked() {
                // In a real implementation, you'd open a file dialog here
                self.add_message(Message::Info("File browser not implemented in this demo".to_string()));
            }
            
            ui.add_space(20.0);
            
            // Processing status
            match &self.processing_state {
                ProcessingState::Idle => {
                    ui.label("Ready to process files");
                }
                ProcessingState::Processing => {
                    ui.label("⏳ Processing...");
                }
                ProcessingState::Completed => {
                    ui.label("✅ Processing completed");
                }
                ProcessingState::Error(error) => {
                    ui.label(format!("❌ Error: {}", error));
                }
            }
            
            // Statistics
            if let Some(analytics) = &self.analytics {
                ui.add_space(20.0);
                ui.heading("Quick Statistics");
                
                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.label(format!("Total Calls: {}", analytics.total_calls));
                        ui.label(format!("Incoming: {}", analytics.incoming_calls));
                        ui.label(format!("Outgoing: {}", analytics.outgoing_calls));
                    });
                    ui.vertical(|ui| {
                        ui.label(format!("Unique Numbers: {}", analytics.unique_numbers));
                        ui.label(format!("Total Duration: {:.1} min", analytics.total_duration_minutes));
                        ui.label(format!("Avg Duration: {:.1} min", analytics.average_call_duration));
                    });
                });
            }
        });
    }
    
    fn render_call_records(&mut self, ui: &mut egui::Ui) {
        if self.call_records.is_empty() {
            ui.centered_and_justified(|ui| {
                ui.label("No call records loaded. Please process an XML file first.");
            });
            return;
        }
        
        ui.horizontal(|ui| {
            ui.label(format!("Showing {} call records", self.call_records.len()));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Export to Excel").clicked() {
                    self.export_to_excel();
                }
            });
        });
        
        egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
            egui::Grid::new("call_records").striped(true).show(ui, |ui| {
                // Headers
                ui.strong("Direction");
                ui.strong("Remote Number");
                ui.strong("Normalized");
                ui.strong("Date");
                ui.strong("Time");
                ui.strong("Duration (min)");
                ui.end_row();
                
                // Data (show first 100 records)
                for record in self.call_records.iter().take(100) {
                    ui.label(&record.message_direction);
                    ui.label(&record.remote_number);
                    ui.label(&record.normalized_number);
                    ui.label(&record.date);
                    ui.label(&record.time);
                    ui.label(format!("{:.2}", record.duration_minutes));
                    ui.end_row();
                }
            });
            
            if self.call_records.len() > 100 {
                ui.label(format!("... and {} more records", self.call_records.len() - 100));
            }
        });
    }
    
    fn render_analytics(&mut self, ui: &mut egui::Ui) {
        if let Some(analytics) = &self.analytics {
            ui.vertical(|ui| {
                ui.heading("Analytics Dashboard");
                
                // Summary cards
                ui.horizontal(|ui| {
                    self.render_stat_card(ui, "Total Calls", &analytics.total_calls.to_string(), "📞");
                    self.render_stat_card(ui, "Incoming", &analytics.incoming_calls.to_string(), "📥");
                    self.render_stat_card(ui, "Outgoing", &analytics.outgoing_calls.to_string(), "📤");
                    self.render_stat_card(ui, "Unique Numbers", &analytics.unique_numbers.to_string(), "👥");
                });
                
                ui.horizontal(|ui| {
                    self.render_stat_card(ui, "Total Duration", &format!("{:.1} min", analytics.total_duration_minutes), "⏱️");
                    self.render_stat_card(ui, "Avg Duration", &format!("{:.1} min", analytics.average_call_duration), "📊");
                });
                
                ui.add_space(20.0);
                
                // Most frequent numbers
                ui.heading("Most Frequent Numbers");
                egui::Grid::new("frequent_numbers").striped(true).show(ui, |ui| {
                    ui.strong("Rank");
                    ui.strong("Phone Number");
                    ui.strong("Call Count");
                    ui.end_row();
                    
                    for (i, (number, count)) in analytics.most_frequent_numbers.iter().enumerate() {
                        ui.label(format!("{}", i + 1));
                        ui.label(number);
                        ui.label(count.to_string());
                        ui.end_row();
                    }
                });
                
                ui.add_space(20.0);
                
                // Calls by day
                ui.heading("Calls by Day");
                let mut sorted_days: Vec<_> = analytics.calls_by_day.iter().collect();
                sorted_days.sort_by(|a, b| a.0.cmp(b.0));
                
                egui::Grid::new("calls_by_day").striped(true).show(ui, |ui| {
                    ui.strong("Date");
                    ui.strong("Call Count");
                    ui.end_row();
                    
                    for (day, count) in sorted_days {
                        ui.label(day);
                        ui.label(count.to_string());
                        ui.end_row();
                    }
                });
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No analytics available. Please process an XML file first.");
            });
        }
    }
    
    fn render_summary(&mut self, ui: &mut egui::Ui) {
        if let Some(analytics) = &self.analytics {
            let report = AnalyticsEngine::generate_summary_report(analytics);
            
            ui.horizontal(|ui| {
                ui.label("Summary Report");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("Copy to Clipboard").clicked() {
                        ui.output_mut(|o| o.copied_text = report.clone());
                        self.add_message(Message::Success("Report copied to clipboard".to_string()));
                    }
                });
            });
            
            egui::ScrollArea::vertical().max_height(600.0).show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut report.as_str()).desired_width(f32::INFINITY));
            });
        } else {
            ui.centered_and_justified(|ui| {
                ui.label("No summary available. Please process an XML file first.");
            });
        }
    }
    
    fn render_stat_card(&self, ui: &mut egui::Ui, title: &str, value: &str, icon: &str) {
        ui.vertical(|ui| {
            ui.add_space(10.0);
            ui.horizontal(|ui| {
                ui.label(icon);
                ui.label(title);
            });
            ui.heading(value);
            ui.add_space(10.0);
        });
    }
} 