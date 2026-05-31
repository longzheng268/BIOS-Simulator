// Room scene — the main exploration area (attic room)
//
// Draws the room with interactive objects using simple pixel-art style shapes.
// Objects: desk, CRT monitor, keyboard, bookshelf, window, lamp, drawers, telephone

use macroquad::prelude::*;

/// Interactive hotspot in the room
pub struct Hotspot {
    pub id: String,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub label_zh: String,
    pub label_en: String,
    pub hover: bool,
}

impl Hotspot {
    pub fn contains(&self, mx: f32, my: f32) -> bool {
        mx >= self.x && mx <= self.x + self.w && my >= self.y && my <= self.y + self.h
    }

    pub fn label(&self, lang: crate::config::Language) -> &str {
        match lang {
            crate::config::Language::Chinese => &self.label_zh,
            crate::config::Language::English => &self.label_en,
        }
    }
}

/// Room scene state
pub struct Room {
    pub hotspots: Vec<Hotspot>,
    pub hovered_id: Option<String>,
    /// Room x offset (centered in CRT area)
    pub offset_x: f32,
    pub offset_y: f32,
}

impl Room {
    pub fn new() -> Self {
        // Define interactive hotspots (relative to room origin)
        let hotspots = vec![
            Hotspot {
                id: "monitor".to_string(),
                x: 400.0, y: 100.0, w: 480.0, h: 360.0,
                label_zh: "CRT 显示器".to_string(),
                label_en: "CRT Monitor".to_string(),
                hover: false,
            },
            Hotspot {
                id: "keyboard".to_string(),
                x: 420.0, y: 480.0, w: 440.0, h: 60.0,
                label_zh: "键盘".to_string(),
                label_en: "Keyboard".to_string(),
                hover: false,
            },
            Hotspot {
                id: "bookshelf".to_string(),
                x: 40.0, y: 80.0, w: 120.0, h: 400.0,
                label_zh: "书架".to_string(),
                label_en: "Bookshelf".to_string(),
                hover: false,
            },
            Hotspot {
                id: "window".to_string(),
                x: 900.0, y: 80.0, w: 300.0, h: 280.0,
                label_zh: "窗户".to_string(),
                label_en: "Window".to_string(),
                hover: false,
            },
            Hotspot {
                id: "drawer".to_string(),
                x: 400.0, y: 560.0, w: 200.0, h: 80.0,
                label_zh: "抽屉".to_string(),
                label_en: "Drawer".to_string(),
                hover: false,
            },
            Hotspot {
                id: "lamp".to_string(),
                x: 340.0, y: 200.0, w: 40.0, h: 120.0,
                label_zh: "台灯".to_string(),
                label_en: "Desk Lamp".to_string(),
                hover: false,
            },
            Hotspot {
                id: "telephone".to_string(),
                x: 700.0, y: 500.0, w: 80.0, h: 60.0,
                label_zh: "电话".to_string(),
                label_en: "Telephone".to_string(),
                hover: false,
            },
            Hotspot {
                id: "calendar".to_string(),
                x: 180.0, y: 100.0, w: 100.0, h: 120.0,
                label_zh: "日历".to_string(),
                label_en: "Calendar".to_string(),
                hover: false,
            },
        ];

        Self {
            hotspots,
            hovered_id: None,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// Update hover state based on mouse position
    pub fn update(&mut self, mx: f32, my: f32) {
        // Adjust mouse position for room offset
        let rx = mx - self.offset_x;
        let ry = my - self.offset_y;

        self.hovered_id = None;
        for hs in &mut self.hotspots {
            hs.hover = hs.contains(rx, ry);
            if hs.hover {
                self.hovered_id = Some(hs.id.clone());
            }
        }
    }

    /// Check if a hotspot was clicked
    pub fn clicked(&self, mx: f32, my: f32) -> Option<String> {
        let rx = mx - self.offset_x;
        let ry = my - self.offset_y;
        for hs in &self.hotspots {
            if hs.contains(rx, ry) {
                return Some(hs.id.clone());
            }
        }
        None
    }

    /// Draw the room scene
    pub fn draw(&self) {
        let ox = self.offset_x;
        let oy = self.offset_y;

        // Floor
        draw_rectangle(ox, oy + 500.0, 1280.0, 220.0, Color::new(0.25, 0.20, 0.15, 1.0));

        // Back wall
        draw_rectangle(ox, oy, 1280.0, 500.0, Color::new(0.35, 0.30, 0.25, 1.0));

        // Window
        draw_rectangle(ox + 900.0, oy + 80.0, 300.0, 280.0, Color::new(0.1, 0.15, 0.3, 1.0));
        // Window frame
        draw_rectangle_lines(ox + 900.0, oy + 80.0, 300.0, 280.0, 4.0, Color::new(0.4, 0.35, 0.3, 1.0));
        // Window cross
        draw_rectangle(ox + 1048.0, oy + 80.0, 4.0, 280.0, Color::new(0.4, 0.35, 0.3, 1.0));
        draw_rectangle(ox + 900.0, oy + 218.0, 300.0, 4.0, Color::new(0.4, 0.35, 0.3, 1.0));
        // Moonlight
        draw_circle(ox + 1050.0, oy + 150.0, 30.0, Color::new(0.8, 0.85, 1.0, 0.6));

        // Bookshelf
        draw_rectangle(ox + 40.0, oy + 80.0, 120.0, 400.0, Color::new(0.3, 0.2, 0.12, 1.0));
        for i in 0..4 {
            let shelf_y = oy + 80.0 + i as f32 * 100.0;
            draw_rectangle(ox + 40.0, shelf_y + 95.0, 120.0, 5.0, Color::new(0.25, 0.18, 0.1, 1.0));
            // Books
            for j in 0..5 {
                let book_x = ox + 48.0 + j as f32 * 20.0;
                let book_h = 60.0 + (j * 17 % 30) as f32;
                let colors = [
                    Color::new(0.6, 0.2, 0.2, 1.0),
                    Color::new(0.2, 0.4, 0.6, 1.0),
                    Color::new(0.3, 0.5, 0.3, 1.0),
                    Color::new(0.6, 0.5, 0.2, 1.0),
                    Color::new(0.5, 0.3, 0.5, 1.0),
                ];
                draw_rectangle(book_x, shelf_y + 95.0 - book_h, 16.0, book_h, colors[j]);
            }
        }

        // Desk
        draw_rectangle(ox + 340.0, oy + 460.0, 600.0, 20.0, Color::new(0.4, 0.3, 0.18, 1.0));
        // Desk legs
        draw_rectangle(ox + 360.0, oy + 480.0, 10.0, 120.0, Color::new(0.35, 0.25, 0.15, 1.0));
        draw_rectangle(ox + 900.0, oy + 480.0, 10.0, 120.0, Color::new(0.35, 0.25, 0.15, 1.0));

        // CRT Monitor (simplified)
        draw_rectangle(ox + 480.0, oy + 120.0, 320.0, 260.0, Color::new(0.2, 0.2, 0.2, 1.0));
        // Screen bezel
        draw_rectangle(ox + 490.0, oy + 130.0, 300.0, 240.0, Color::new(0.05, 0.05, 0.08, 1.0));
        // Screen glow
        draw_rectangle(ox + 495.0, oy + 135.0, 290.0, 230.0, Color::new(0.02, 0.08, 0.02, 1.0));
        // Monitor base
        draw_rectangle(ox + 560.0, oy + 380.0, 160.0, 30.0, Color::new(0.2, 0.2, 0.2, 1.0));
        // Power LED
        draw_circle(ox + 640.0, oy + 395.0, 3.0, Color::new(0.0, 0.8, 0.0, 1.0));

        // Keyboard
        draw_rectangle(ox + 420.0, oy + 480.0, 440.0, 60.0, Color::new(0.22, 0.22, 0.22, 1.0));
        for row in 0..3 {
            for col in 0..12 {
                let kx = ox + 428.0 + col as f32 * 34.0;
                let ky = oy + 486.0 + row as f32 * 16.0;
                draw_rectangle(kx, ky, 28.0, 12.0, Color::new(0.3, 0.3, 0.3, 1.0));
            }
        }

        // Desk lamp
        draw_rectangle(ox + 350.0, oy + 200.0, 20.0, 120.0, Color::new(0.5, 0.5, 0.5, 1.0));
        // Lamp shade
        draw_triangle(
            vec2(ox + 320.0, oy + 200.0),
            vec2(ox + 400.0, oy + 200.0),
            vec2(ox + 360.0, oy + 160.0),
            Color::new(0.8, 0.7, 0.3, 1.0),
        );
        // Lamp glow
        draw_circle(ox + 360.0, oy + 180.0, 50.0, Color::new(1.0, 0.9, 0.6, 0.05));

        // Telephone
        draw_rectangle(ox + 700.0, oy + 500.0, 80.0, 40.0, Color::new(0.15, 0.15, 0.15, 1.0));
        draw_rectangle(ox + 710.0, oy + 505.0, 60.0, 15.0, Color::new(0.2, 0.2, 0.2, 1.0));

        // Drawer
        draw_rectangle(ox + 400.0, oy + 560.0, 200.0, 80.0, Color::new(0.35, 0.25, 0.15, 1.0));
        draw_rectangle(ox + 480.0, oy + 590.0, 40.0, 10.0, Color::new(0.6, 0.5, 0.3, 1.0));

        // Calendar on wall
        draw_rectangle(ox + 180.0, oy + 100.0, 100.0, 120.0, Color::new(0.9, 0.9, 0.85, 1.0));
        draw_rectangle(ox + 180.0, oy + 100.0, 100.0, 30.0, Color::new(0.7, 0.2, 0.2, 1.0));

        // Highlight hovered hotspot
        for hs in &self.hotspots {
            if hs.hover {
                draw_rectangle_lines(
                    ox + hs.x, oy + hs.y, hs.w, hs.h,
                    2.0, Color::new(1.0, 1.0, 0.0, 0.6),
                );
            }
        }
    }
}
