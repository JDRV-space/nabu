use leptos::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Document {
    pub id: String,
    pub title: String,
    pub author: Option<String>,
    pub content: String,
    pub file_type: String,
    pub progress: f64,
    pub last_position: usize,
    pub last_read: Option<String>,
    pub created_at: String,
    pub word_count: usize,
}

impl Document {
    pub fn new(title: String, content: String, file_type: String) -> Self {
        let word_count = content.split_whitespace().count();
        Self {
            id: uuid(),
            title,
            author: None,
            content,
            file_type,
            progress: 0.0,
            last_position: 0,
            last_read: None,
            created_at: chrono::Utc::now().to_rfc3339(),
            word_count,
        }
    }

    pub fn words(&self) -> Vec<String> {
        self.content
            .split_whitespace()
            .map(|s| s.to_string())
            .collect()
    }
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct ReadingStats {
    pub total_words: u64,
    pub total_hours: f64,
    pub avg_wpm: u32,
    pub streak_days: u32,
    pub words_change: Option<i32>,
    pub hours_change: Option<i32>,
    pub wpm_change: Option<i32>,
    pub daily_activity: Vec<f64>,
    pub weekly_wpm: Vec<u32>,
}

#[derive(Clone, Copy)]
pub struct AppState {
    pub documents: RwSignal<Vec<Document>>,
    pub current_document: RwSignal<Option<Document>>,
    pub current_words: RwSignal<Vec<String>>,
    pub wpm: RwSignal<u32>,
    pub font_size: RwSignal<String>,
    pub theme: RwSignal<String>,
    pub chunk_size: RwSignal<u8>,
    pub bionic_mode: RwSignal<bool>,
    pub punctuation_pause: RwSignal<bool>,
    pub speed_ramping: RwSignal<bool>,
    pub reading_stats: RwSignal<ReadingStats>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            documents: RwSignal::new(Vec::new()),
            current_document: RwSignal::new(None),
            current_words: RwSignal::new(Vec::new()),
            wpm: RwSignal::new(300),
            font_size: RwSignal::new("L".to_string()),
            theme: RwSignal::new("dark".to_string()),
            chunk_size: RwSignal::new(1),
            bionic_mode: RwSignal::new(false),
            punctuation_pause: RwSignal::new(true),
            speed_ramping: RwSignal::new(false),
            reading_stats: RwSignal::new(ReadingStats::default()),
        }
    }

    pub fn add_document(&self, doc: Document) {
        self.documents.update(|docs| docs.push(doc));
    }

    pub fn set_current_document(&self, doc: Document) {
        let words = doc.words();
        self.current_words.set(words);
        self.current_document.set(Some(doc));
    }

    pub fn set_wpm(&self, value: u32) {
        self.wpm.set(value.clamp(100, 1000));
    }

    pub fn adjust_wpm(&self, delta: i32) {
        let current = self.wpm.get() as i32;
        let new_value = (current + delta).clamp(100, 1000) as u32;
        self.wpm.set(new_value);
    }

    pub fn set_font_size(&self, size: String) {
        self.font_size.set(size);
    }

    pub fn set_theme(&self, theme: String) {
        self.theme.set(theme);
    }

    pub fn set_chunk_size(&self, size: u8) {
        // Valid chunk sizes: 1, 3, 5, 10, 20
        let valid_sizes = [1, 3, 5, 10, 20];
        let clamped = if valid_sizes.contains(&size) {
            size
        } else {
            // Find nearest valid size
            *valid_sizes.iter().min_by_key(|&&s| (s as i32 - size as i32).abs()).unwrap_or(&1)
        };
        self.chunk_size.set(clamped);
    }

    pub fn toggle_bionic(&self) {
        self.bionic_mode.update(|b| *b = !*b);
    }

    pub fn toggle_punctuation_pause(&self) {
        self.punctuation_pause.update(|p| *p = !*p);
    }

    pub fn toggle_speed_ramping(&self) {
        self.speed_ramping.update(|s| *s = !*s);
    }
}

fn uuid() -> String {
    let mut bytes = [0u8; 16];
    if getrandom::getrandom(&mut bytes).is_err() {
        // Fallback to timestamp-based ID
        let now = js_sys::Date::now() as u64;
        bytes[0..8].copy_from_slice(&now.to_le_bytes());
        bytes[8..16].copy_from_slice(&now.wrapping_mul(31337).to_le_bytes());
    }
    format!(
        "{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]),
        u16::from_le_bytes([bytes[4], bytes[5]]),
        u16::from_le_bytes([bytes[6], bytes[7]]),
        u16::from_le_bytes([bytes[8], bytes[9]]),
        u64::from_le_bytes([bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15], 0, 0])
            & 0xffffffffffff
    )
}
