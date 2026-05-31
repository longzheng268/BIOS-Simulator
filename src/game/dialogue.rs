// Dialogue system — runtime dialogue engine
//
// Loads dialogue data from audio_gen/scripts/game_script.json
// Audio files are in audio_gen/output/ (140 bilingual WAV files)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::config::Language;
use crate::i18n;

/// A single dialogue segment (one line from one character)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueSegment {
    pub id: String,
    pub character: String,
    pub text: String,       // Chinese text
    pub text_en: String,    // English text
    #[serde(default)]
    pub context: String,
}

/// A chapter containing dialogue segments
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub title: String,
    pub title_zh: String,
    pub segments: Vec<DialogueSegment>,
}

/// Full game script from game_script.json
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameScript {
    pub metadata: serde_json::Value,
    pub chapters: HashMap<String, Chapter>,
}

/// Runtime dialogue state
pub struct DialogueEngine {
    pub script: Option<GameScript>,
    pub current_chapter: Option<String>,
    pub current_segment_index: usize,
    pub is_active: bool,
    pub display_text: String,
    pub target_text: String,
    pub char_index: usize,
    pub typewriter_timer: f64,
    pub waiting_for_input: bool,
}

impl DialogueEngine {
    pub fn new() -> Self {
        Self {
            script: None,
            current_chapter: None,
            current_segment_index: 0,
            is_active: false,
            display_text: String::new(),
            target_text: String::new(),
            char_index: 0,
            typewriter_timer: 0.0,
            waiting_for_input: false,
        }
    }

    /// Load from audio_gen/scripts/game_script.json
    pub fn load(&mut self, path: &str) -> Result<(), String> {
        let data = std::fs::read_to_string(path)
            .map_err(|e| format!("Failed to read {}: {}", path, e))?;
        let script: GameScript = serde_json::from_str(&data)
            .map_err(|e| format!("Failed to parse: {}", e))?;
        let chapter_count = script.chapters.len();
        let segment_count: usize = script.chapters.values().map(|c| c.segments.len()).sum();
        println!("Loaded game script: {} chapters, {} segments", chapter_count, segment_count);
        self.script = Some(script);
        Ok(())
    }

    /// Start a chapter by key (e.g., "chapter_1_player_monologue")
    pub fn start_chapter(&mut self, chapter_key: &str) {
        if let Some(ref script) = self.script {
            if script.chapters.contains_key(chapter_key) {
                self.current_chapter = Some(chapter_key.to_string());
                self.current_segment_index = 0;
                self.is_active = true;
                self.start_current_segment();
            }
        }
    }

    fn start_current_segment(&mut self) {
        if let Some(ref script) = self.script {
            if let Some(ref chapter_key) = self.current_chapter {
                if let Some(chapter) = script.chapters.get(chapter_key) {
                    if let Some(segment) = chapter.segments.get(self.current_segment_index) {
                        self.target_text = segment.text.clone();
                        self.display_text.clear();
                        self.char_index = 0;
                        self.typewriter_timer = 0.0;
                        self.waiting_for_input = false;
                    }
                }
            }
        }
    }

    pub fn current_character(&self, lang: Language) -> String {
        if let Some(ref script) = self.script {
            if let Some(ref chapter_key) = self.current_chapter {
                if let Some(chapter) = script.chapters.get(chapter_key) {
                    if let Some(segment) = chapter.segments.get(self.current_segment_index) {
                        return i18n::character_name(&segment.character, lang).to_string();
                    }
                }
            }
        }
        String::new()
    }

    pub fn current_text(&self, lang: Language) -> String {
        if let Some(ref script) = self.script {
            if let Some(ref chapter_key) = self.current_chapter {
                if let Some(chapter) = script.chapters.get(chapter_key) {
                    if let Some(segment) = chapter.segments.get(self.current_segment_index) {
                        return match lang {
                            Language::English => segment.text_en.clone(),
                            Language::Chinese => segment.text.clone(),
                        };
                    }
                }
            }
        }
        String::new()
    }

    /// Audio file path for current segment
    pub fn current_audio_path(&self, lang: Language) -> Option<String> {
        if let Some(ref script) = self.script {
            if let Some(ref chapter_key) = self.current_chapter {
                if let Some(chapter) = script.chapters.get(chapter_key) {
                    if let Some(segment) = chapter.segments.get(self.current_segment_index) {
                        let lang_suffix = match lang {
                            Language::Chinese => "zh",
                            Language::English => "en",
                        };
                        return Some(format!("audio_gen/output/{}/{}_{}.wav",
                            chapter_key, segment.id, lang_suffix));
                    }
                }
            }
        }
        None
    }

    /// Update typewriter effect
    pub fn update(&mut self, dt: f64) {
        if !self.is_active || self.waiting_for_input {
            return;
        }
        self.typewriter_timer += dt;
        let interval = 1.0 / 30.0; // 30 chars/sec
        while self.typewriter_timer >= interval && self.char_index < self.target_text.len() {
            if let Some((next, _)) = self.target_text[self.char_index..].char_indices().nth(1) {
                self.char_index += next;
            } else {
                self.char_index = self.target_text.len();
            }
            self.typewriter_timer -= interval;
        }
        self.display_text = self.target_text[..self.char_index].to_string();
        if self.char_index >= self.target_text.len() {
            self.waiting_for_input = true;
        }
    }

    /// Advance to next segment
    pub fn advance(&mut self) {
        if !self.is_active { return; }
        if !self.waiting_for_input {
            self.display_text = self.target_text.clone();
            self.char_index = self.target_text.len();
            self.waiting_for_input = true;
            return;
        }
        self.current_segment_index += 1;
        if let Some(ref script) = self.script {
            if let Some(ref chapter_key) = self.current_chapter {
                if let Some(chapter) = script.chapters.get(chapter_key) {
                    if self.current_segment_index >= chapter.segments.len() {
                        self.is_active = false;
                        self.current_chapter = None;
                        return;
                    }
                }
            }
        }
        self.start_current_segment();
    }

    pub fn skip(&mut self) {
        self.is_active = false;
        self.current_chapter = None;
    }

    /// List available chapter keys
    pub fn chapter_keys(&self) -> Vec<String> {
        self.script.as_ref()
            .map(|s| s.chapters.keys().cloned().collect())
            .unwrap_or_default()
    }
}
