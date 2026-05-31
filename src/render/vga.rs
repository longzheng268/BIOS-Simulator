/// VGA 80x25 text mode renderer
///
/// Renders characters using an embedded 8x16 bitmap font.
/// Each character has a foreground and background color from the VGA 16-color palette.

use macroquad::prelude::*;
use crate::config::{VGA_COLS, VGA_ROWS, CHAR_WIDTH, CHAR_HEIGHT};

// ─── VGA 16-color palette (standard CGA/VGA colors) ───

/// VGA palette: 16 colors as (R, G, B)
pub const VGA_PALETTE: [(u8, u8, u8); 16] = [
    (0, 0, 0),         // 0  Black
    (0, 0, 170),       // 1  Blue
    (0, 170, 0),       // 2  Green
    (0, 170, 170),     // 3  Cyan
    (170, 0, 0),       // 4  Red
    (170, 0, 170),     // 5  Magenta
    (170, 85, 0),      // 6  Brown / Dark Yellow
    (170, 170, 170),   // 7  Light Gray
    (85, 85, 85),      // 8  Dark Gray
    (85, 85, 255),     // 9  Light Blue
    (85, 255, 85),     // 10 Light Green
    (85, 255, 255),    // 11 Light Cyan
    (255, 85, 85),     // 12 Light Red
    (255, 85, 255),    // 13 Light Magenta
    (255, 255, 85),    // 14 Yellow
    (255, 255, 255),   // 15 White
];

fn palette_color(index: u8) -> Color {
    let (r, g, b) = VGA_PALETTE[(index & 0x0F) as usize];
    Color::new(r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0, 1.0)
}

// ─── VGA character cell ───

#[derive(Clone, Copy, Debug)]
pub struct VgaChar {
    pub ch: u8,       // Character code (CP437 index, 0-255)
    pub fg: u8,       // Foreground color index (0-15)
    pub bg: u8,       // Background color index (0-15)
}

impl Default for VgaChar {
    fn default() -> Self {
        Self { ch: b' ', fg: 7, bg: 0 } // White on black
    }
}

// ─── VGA text buffer ───

pub struct VgaBuffer {
    pub cells: [[VgaChar; VGA_COLS]; VGA_ROWS],
    pub cursor_row: usize,
    pub cursor_col: usize,
    pub cursor_visible: bool,
}

impl VgaBuffer {
    pub fn new() -> Self {
        Self {
            cells: [[VgaChar::default(); VGA_COLS]; VGA_ROWS],
            cursor_row: 0,
            cursor_col: 0,
            cursor_visible: true,
        }
    }

    /// Clear the entire screen with the given color
    pub fn clear(&mut self, fg: u8, bg: u8) {
        for row in &mut self.cells {
            for cell in row.iter_mut() {
                cell.ch = b' ';
                cell.fg = fg;
                cell.bg = bg;
            }
        }
        self.cursor_row = 0;
        self.cursor_col = 0;
    }

    /// Write a single character at the current cursor position, advancing the cursor
    pub fn put_char(&mut self, ch: u8, fg: u8, bg: u8) {
        if ch == b'\n' {
            self.newline();
            return;
        }
        if ch == b'\r' {
            self.cursor_col = 0;
            return;
        }
        if ch == b'\t' {
            let spaces = 8 - (self.cursor_col % 8);
            for _ in 0..spaces {
                self.put_char(b' ', fg, bg);
            }
            return;
        }

        if self.cursor_col < VGA_COLS && self.cursor_row < VGA_ROWS {
            self.cells[self.cursor_row][self.cursor_col] = VgaChar { ch, fg, bg };
            self.cursor_col += 1;
        }
        if self.cursor_col >= VGA_COLS {
            self.newline();
        }
    }

    /// Write a string at the current cursor position
    pub fn put_str(&mut self, s: &str, fg: u8, bg: u8) {
        for ch in s.chars() {
            if ch.is_ascii() {
                self.put_char(ch as u8, fg, bg);
            } else {
                // For non-ASCII (e.g., Chinese), use '?' as placeholder
                // Full CJK rendering will use a separate font system
                self.put_char(b'?', fg, bg);
            }
        }
    }

    /// Move cursor to the next line, scrolling if necessary
    pub fn newline(&mut self) {
        self.cursor_col = 0;
        self.cursor_row += 1;
        if self.cursor_row >= VGA_ROWS {
            self.scroll_up();
            self.cursor_row = VGA_ROWS - 1;
        }
    }

    /// Scroll the entire buffer up by one line
    pub fn scroll_up(&mut self) {
        for row in 1..VGA_ROWS {
            for col in 0..VGA_COLS {
                self.cells[row - 1][col] = self.cells[row][col];
            }
        }
        // Clear the bottom row
        for col in 0..VGA_COLS {
            self.cells[VGA_ROWS - 1][col] = VgaChar::default();
        }
    }

    /// Set cursor position
    pub fn set_cursor(&mut self, row: usize, col: usize) {
        self.cursor_row = row.min(VGA_ROWS - 1);
        self.cursor_col = col.min(VGA_COLS - 1);
    }
}

// ─── VGA font (embedded 8x16 bitmap) ───

include!("vga_font_data.rs");

// ─── VGA renderer ───

pub struct VgaRenderer {
    // Future optimization: cache character textures instead of redrawing each frame
}

impl VgaRenderer {
    pub fn new() -> Self {
        Self {}
    }

    /// Render the VGA buffer to the screen.
    /// `x`, `y` = top-left pixel position of the VGA display area
    /// `scale` = pixel scale factor (2x = each VGA pixel becomes 2x2 screen pixels)
    pub fn draw(&self, buffer: &VgaBuffer, x: f32, y: f32, scale: f32, show_cursor: bool, cursor_blink_on: bool) {
        let cw = CHAR_WIDTH * scale;
        let ch = CHAR_HEIGHT * scale;

        for row in 0..VGA_ROWS {
            for col in 0..VGA_COLS {
                let cell = buffer.cells[row][col];
                let px = x + col as f32 * cw;
                let py = y + row as f32 * ch;

                // Draw background
                let bg_color = palette_color(cell.bg);
                draw_rectangle(px, py, cw, ch, bg_color);

                // Draw character pixels
                let fg_color = palette_color(cell.fg);
                self.draw_char_glyph(cell.ch, px, py, scale, fg_color);
            }
        }

        // Draw cursor
        if show_cursor && cursor_blink_on {
            let cx = x + buffer.cursor_col as f32 * cw;
            let cy = y + buffer.cursor_row as f32 * ch;
            // Cursor is typically the bottom 2 rows of the character cell
            let cursor_h = 2.0 * scale;
            let cursor_y = cy + ch - cursor_h;
            draw_rectangle(cx, cursor_y, cw, cursor_h, palette_color(7)); // White cursor
        }
    }

    /// Draw a single character's glyph at the given position
    fn draw_char_glyph(&self, ch: u8, x: f32, y: f32, scale: f32, color: Color) {
        let font_data = get_char_bitmap(ch);
        for row in 0..16u8 {
            let byte = font_data[row as usize];
            for bit in 0..8u8 {
                if byte & (0x80 >> bit) != 0 {
                    let px = x + bit as f32 * scale;
                    let py = y + row as f32 * scale;
                    draw_rectangle(px, py, scale, scale, color);
                }
            }
        }
    }
}

/// Get the 16-byte bitmap for a character (8 pixels wide × 16 pixels tall)
fn get_char_bitmap(ch: u8) -> [u8; 16] {
    let idx = ch as usize * 16;
    if idx + 16 <= VGA_FONT_DATA.len() {
        let mut buf = [0u8; 16];
        buf.copy_from_slice(&VGA_FONT_DATA[idx..idx + 16]);
        buf
    } else {
        // Fallback: empty character
        [0u8; 16]
    }
}
