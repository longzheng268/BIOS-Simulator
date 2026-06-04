// Audio player — loads and plays WAV files from assets/audio/
//
// Queue-based: call request_play() to queue a sound, then call update()
// each frame to process the queue (async loading).

use macroquad::audio::{load_sound, play_sound, stop_sound, Sound, PlaySoundParams};
use std::collections::HashMap;
use crate::config::Language;

pub struct AudioPlayer {
    cache: HashMap<String, Sound>,
    current: Option<Sound>,
    volume: f32,
    enabled: bool,
    pending: Option<String>,  // Path waiting to be loaded
    ready: Option<Sound>,     // Loaded sound ready to play
}

impl AudioPlayer {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
            current: None,
            volume: 0.8,
            enabled: true,
            pending: None,
            ready: None,
        }
    }

    /// Queue a sound to be played. Stops current sound immediately.
    pub fn request_play(&mut self, path: &str) {
        if !self.enabled { return; }
        self.stop();
        self.pending = Some(path.to_string());
        self.ready = None;
    }

    /// Process the play queue. Call this each frame from an async context.
    pub async fn update(&mut self) {
        // If we have a loaded sound ready, play it
        if let Some(sound) = self.ready.take() {
            play_sound(&sound, PlaySoundParams {
                looped: false,
                volume: self.volume,
            });
            self.current = Some(sound);
        }

        // If we have a pending request, load it
        if let Some(ref path) = self.pending.clone() {
            let sound = if let Some(cached) = self.cache.get(path) {
                Some(cached.clone())
            } else {
                match load_sound(path).await {
                    Ok(sound) => {
                        self.cache.insert(path.clone(), sound.clone());
                        Some(sound)
                    }
                    Err(e) => {
                        // WAV file not found is OK — dialogue works without audio
                        if !path.contains("_en.wav") && !path.contains("_zh.wav") {
                            eprintln!("Audio load failed: {} — {}", path, e);
                        }
                        None
                    }
                }
            };
            self.pending = None;
            self.ready = sound;
        }
    }

    pub fn stop(&mut self) {
        if let Some(ref sound) = self.current {
            stop_sound(sound);
        }
        self.current = None;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
        if !self.enabled { self.stop(); }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Build the audio file path for a dialogue segment
    pub fn dialogue_audio_path(chapter_key: &str, segment_id: &str, lang: Language) -> String {
        let lang_suffix = match lang {
            Language::Chinese => "zh",
            Language::English => "en",
        };
        format!("assets/audio/{}/{}_{}.wav", chapter_key, segment_id, lang_suffix)
    }
}
