/// Global game configuration

/// Window resolution (product design spec: 1280x720)
pub const WINDOW_WIDTH: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;
pub const WINDOW_TITLE: &str = "x86 BIOS Simulator";

/// VGA text mode: 80 columns x 25 rows
pub const VGA_COLS: usize = 80;
pub const VGA_ROWS: usize = 25;

/// VGA character cell size in pixels
pub const CHAR_WIDTH: f32 = 8.0;
pub const CHAR_HEIGHT: f32 = 16.0;

/// CRT display area (centered in window, with room for the "monitor" frame)
/// The VGA content renders at 2x scale inside the CRT area
pub const CRT_SCALE: f32 = 2.0;
pub const CRT_CONTENT_W: f32 = VGA_COLS as f32 * CHAR_WIDTH * CRT_SCALE;   // 1280
pub const CRT_CONTENT_H: f32 = VGA_ROWS as f32 * CHAR_HEIGHT * CRT_SCALE;  // 800

/// Cursor blink period (ms) — VGA standard ~53ms
pub const CURSOR_BLINK_MS: u64 = 530;

/// Supported languages
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum Language {
    Chinese,
    English,
}

impl Language {
    pub fn toggle(self) -> Self {
        match self {
            Language::Chinese => Language::English,
            Language::English => Language::Chinese,
        }
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::Chinese
    }
}
