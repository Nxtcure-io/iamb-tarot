//! API client for tarot reading history and analytics

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

const API_BASE_URL: &str = "https://endlessperfect.com/tarot-api/api";

#[derive(Debug, Deserialize)]
pub struct CardInReading {
    pub position: i32,
    pub card_name: String,
    pub label: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReadingHistoryItem {
    pub reading_id: i32,
    pub spread_type: String,
    pub reading_date: String,
    pub card_count: usize,
    pub cards: Vec<CardInReading>,
    pub notes: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct HistoryResponse {
    pub total_readings: usize,
    pub readings: Vec<ReadingHistoryItem>,
}

#[derive(Debug, Deserialize)]
pub struct CardDetail {
    pub position: i32,
    pub card_name: String,
    pub label: Option<String>,
    pub info: Option<String>,
    pub deepinfo: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ReadingDetails {
    pub reading_id: i32,
    pub spread_type: String,
    pub reading_date: String,
    pub notes: Option<String>,
    pub cards: Vec<CardDetail>,
    pub attributes: HashMap<String, HashMap<String, i32>>,
}

#[derive(Debug, Deserialize)]
pub struct AttributeFrequency {
    pub attribute_type: String,
    pub total_count: i32,
    pub frequencies: HashMap<String, i32>,
    pub percentages: HashMap<String, f64>,
}

#[derive(Debug, Deserialize)]
pub struct TopAttribute {
    pub value: String,
    pub count: i32,
}

#[derive(Debug, Serialize)]
pub struct CardData {
    pub position: i32,
    pub card_name: String,
    pub card_label: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ReadingCreate {
    pub matrix_id: String,
    pub room_id: Option<String>,
    pub spread_type: String,
    pub cards: Vec<CardData>,
    pub notes: Option<String>,
    pub is_private: bool,
}

#[derive(Debug, Deserialize)]
pub struct ReadingCreateResponse {
    pub reading_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct SpreadTypeCount {
    #[serde(rename = "type")]
    pub spread_type: String,
    pub count: i32,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsSummary {
    pub total_readings: i32,
    pub total_cards_drawn: i32,
    pub spread_types: Vec<SpreadTypeCount>,
    pub top_attributes: HashMap<String, Vec<TopAttribute>>,
}

/// Get reading history for a user
pub fn get_history(matrix_id: &str) -> Result<HistoryResponse, String> {
    let url = format!("{}/readings/user/{}/history", API_BASE_URL, matrix_id);
    
    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch history: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }
    
    response.json::<HistoryResponse>()
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Get details for a specific reading
pub fn get_reading_details(reading_id: i32) -> Result<ReadingDetails, String> {
    let url = format!("{}/readings/{}/details", API_BASE_URL, reading_id);
    
    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch reading details: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }
    
    response.json::<ReadingDetails>()
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Get attribute frequency distribution
pub fn get_attribute_frequency(matrix_id: &str, attribute_type: &str) -> Result<AttributeFrequency, String> {
    let url = format!("{}/analytics/user/{}/attributes/{}", API_BASE_URL, matrix_id, attribute_type);
    
    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch attribute frequency: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }
    
    response.json::<AttributeFrequency>()
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Get analytics summary
pub fn get_analytics_summary(matrix_id: &str) -> Result<AnalyticsSummary, String> {
    let url = format!("{}/analytics/user/{}/summary", API_BASE_URL, matrix_id);
    
    let response = reqwest::blocking::get(&url)
        .map_err(|e| format!("Failed to fetch analytics summary: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }
    
    response.json::<AnalyticsSummary>()
        .map_err(|e| format!("Failed to parse response: {}", e))
}

/// Generate ASCII bar graph from frequency data
pub fn generate_bar_graph(frequencies: &HashMap<String, i32>, percentages: &HashMap<String, f64>, max_width: usize) -> String {
    if frequencies.is_empty() {
        return "No data available".to_string();
    }
    
    // Sort by count descending
    let mut items: Vec<(&String, &i32)> = frequencies.iter().collect();
    items.sort_by(|a, b| b.1.cmp(a.1));
    
    let max_value = *items[0].1;
    let mut lines = Vec::new();
    
    for (label, &count) in items {
        let percentage = percentages.get(label).unwrap_or(&0.0);
        let bar_width = if max_value > 0 {
            ((count as f64 / max_value as f64) * max_width as f64) as usize
        } else {
            0
        };
        
        let bar = "█".repeat(bar_width);
        let line = format!("{:<15} {} {:.1}% ({})", label, bar, percentage, count);
        lines.push(line);
    }
    
    lines.join("\n")
}

/// Save a new tarot reading
pub fn save_reading(reading: ReadingCreate) -> Result<ReadingCreateResponse, String> {
    let url = format!("{}/readings", API_BASE_URL);
    
    let client = reqwest::blocking::Client::new();
    let response = client.post(&url)
        .json(&reading)
        .send()
        .map_err(|e| format!("Failed to save reading: {}", e))?;
    
    if !response.status().is_success() {
        return Err(format!("API error: {}", response.status()));
    }
    
    response.json::<ReadingCreateResponse>()
        .map_err(|e| format!("Failed to parse response: {}", e))
}
