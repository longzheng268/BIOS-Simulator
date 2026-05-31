// Save/load system — 3 save slots with JSON serialization
//
// Saves: game state, task progress, collected items, dialogue progress, language

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Save data structure
#[derive(Debug, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub slot: u8,
    pub player_name: String,
    pub current_chapter: u8,
    pub play_time_secs: u64,
    pub save_timestamp: String,
    pub language: String,
    pub tasks_completed: Vec<String>,
    pub floppies_collected: Vec<String>,
    pub knowledge_cards: Vec<String>,
    pub dos_current_dir: String,
}

impl SaveData {
    pub fn new(slot: u8) -> Self {
        Self {
            version: 1,
            slot,
            player_name: "Player".to_string(),
            current_chapter: 1,
            play_time_secs: 0,
            save_timestamp: String::new(),
            language: "Chinese".to_string(),
            tasks_completed: Vec::new(),
            floppies_collected: Vec::new(),
            knowledge_cards: Vec::new(),
            dos_current_dir: "C:\\".to_string(),
        }
    }
}

/// Save/load manager
pub struct SaveManager {
    save_dir: PathBuf,
    pub slots: [Option<SaveData>; 3],
}

impl SaveManager {
    pub fn new() -> Self {
        let save_dir = PathBuf::from("saves");
        let mut manager = Self {
            save_dir,
            slots: [None, None, None],
        };
        manager.load_all_slots();
        manager
    }

    /// Get the file path for a save slot
    fn slot_path(&self, slot: u8) -> PathBuf {
        self.save_dir.join(format!("save_{}.json", slot))
    }

    /// Load all save slots from disk
    fn load_all_slots(&mut self) {
        // Create saves directory if it doesn't exist
        let _ = std::fs::create_dir_all(&self.save_dir);

        for slot in 0..3 {
            let path = self.slot_path(slot);
            if path.exists() {
                match std::fs::read_to_string(&path) {
                    Ok(data) => {
                        match serde_json::from_str::<SaveData>(&data) {
                            Ok(save) => self.slots[slot as usize] = Some(save),
                            Err(e) => eprintln!("Failed to parse save slot {}: {}", slot, e),
                        }
                    }
                    Err(e) => eprintln!("Failed to read save slot {}: {}", slot, e),
                }
            }
        }
    }

    /// Save to a specific slot
    pub fn save(&mut self, mut data: SaveData) -> Result<(), String> {
        let slot = data.slot as usize;
        if slot >= 3 {
            return Err("Invalid slot number".to_string());
        }

        // Set timestamp
        data.save_timestamp = chrono_timestamp();

        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| format!("Failed to serialize: {}", e))?;

        std::fs::create_dir_all(&self.save_dir)
            .map_err(|e| format!("Failed to create saves dir: {}", e))?;

        std::fs::write(self.slot_path(data.slot), json)
            .map_err(|e| format!("Failed to write save: {}", e))?;

        self.slots[slot] = Some(data);
        Ok(())
    }

    /// Load from a specific slot
    pub fn load(&self, slot: u8) -> Option<&SaveData> {
        if slot < 3 {
            self.slots[slot as usize].as_ref()
        } else {
            None
        }
    }

    /// Delete a save slot
    pub fn delete(&mut self, slot: u8) {
        if slot < 3 {
            let path = self.slot_path(slot);
            let _ = std::fs::remove_file(path);
            self.slots[slot as usize] = None;
        }
    }

    /// Get summary text for a save slot
    pub fn slot_summary(&self, slot: usize) -> Option<String> {
        self.slots.get(slot)?.as_ref().map(|data| {
            format!(
                "Slot {} — {} — Chapter {} — {} — {}",
                data.slot + 1,
                data.player_name,
                data.current_chapter,
                format_time(data.play_time_secs),
                data.save_timestamp
            )
        })
    }
}

/// Format seconds as HH:MM:SS
fn format_time(secs: u64) -> String {
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{:02}:{:02}:{:02}", h, m, s)
}

/// Simple timestamp (no chrono dependency)
fn chrono_timestamp() -> String {
    // Use a simple format without external dependency
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let days = secs / 86400;
    let year = 1970 + days / 365;
    let month = ((days % 365) / 30) + 1;
    let day = ((days % 365) % 30) + 1;
    format!("{:04}-{:02}-{:02}", year, month, day)
}
