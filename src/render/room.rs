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
        // Hotspots ordered SMALL→LARGE so click() returns the most specific match.
        let hotspots = vec![
            // Small objects first (most specific)
            Hotspot {
                id: "lamp".to_string(),
                x: 340.0, y: 160.0, w: 40.0, h: 80.0,
                label_zh: "台灯".to_string(),
                label_en: "Desk Lamp".to_string(),
                hover: false,
            },
            Hotspot {
                id: "notebook".to_string(),
                x: 500.0, y: 430.0, w: 80.0, h: 30.0,
                label_zh: "笔记本".to_string(),
                label_en: "Notebook".to_string(),
                hover: false,
            },
            Hotspot {
                id: "floppy_box".to_string(),
                x: 820.0, y: 430.0, w: 60.0, h: 30.0,
                label_zh: "软盘盒".to_string(),
                label_en: "Floppy Box".to_string(),
                hover: false,
            },
            Hotspot {
                id: "telephone".to_string(),
                x: 700.0, y: 500.0, w: 80.0, h: 40.0,
                label_zh: "电话".to_string(),
                label_en: "Telephone".to_string(),
                hover: false,
            },
            Hotspot {
                id: "drawer".to_string(),
                x: 400.0, y: 560.0, w: 200.0, h: 60.0,
                label_zh: "抽屉".to_string(),
                label_en: "Drawer".to_string(),
                hover: false,
            },
            Hotspot {
                id: "keyboard".to_string(),
                x: 480.0, y: 480.0, w: 320.0, h: 40.0,
                label_zh: "键盘".to_string(),
                label_en: "Keyboard".to_string(),
                hover: false,
            },
            // Monitor screen area only (not the full bezel)
            Hotspot {
                id: "monitor".to_string(),
                x: 490.0, y: 130.0, w: 300.0, h: 240.0,
                label_zh: "CRT 显示器".to_string(),
                label_en: "CRT Monitor".to_string(),
                hover: false,
            },
            Hotspot {
                id: "poster".to_string(),
                x: 620.0, y: 80.0, w: 120.0, h: 160.0,
                label_zh: "海报".to_string(),
                label_en: "Poster".to_string(),
                hover: false,
            },
            // Medium objects
            Hotspot {
                id: "calendar".to_string(),
                x: 180.0, y: 100.0, w: 100.0, h: 120.0,
                label_zh: "日历".to_string(),
                label_en: "Calendar".to_string(),
                hover: false,
            },
            Hotspot {
                id: "cabinet".to_string(),
                x: 950.0, y: 400.0, w: 100.0, h: 200.0,
                label_zh: "柜子".to_string(),
                label_en: "Cabinet".to_string(),
                hover: false,
            },
            // Floppy disk locations (hidden throughout the room)
            Hotspot {
                id: "floppy_03".to_string(),
                x: 60.0, y: 450.0, w: 30.0, h: 20.0,
                label_zh: "软盘 DISK_03".to_string(),
                label_en: "Floppy DISK_03".to_string(),
                hover: false,
            },
            Hotspot {
                id: "floppy_04".to_string(),
                x: 160.0, y: 200.0, w: 30.0, h: 20.0,
                label_zh: "软盘 DISK_04".to_string(),
                label_en: "Floppy DISK_04".to_string(),
                hover: false,
            },
            Hotspot {
                id: "floppy_05".to_string(),
                x: 960.0, y: 550.0, w: 30.0, h: 20.0,
                label_zh: "软盘 DISK_05".to_string(),
                label_en: "Floppy DISK_05".to_string(),
                hover: false,
            },
            Hotspot {
                id: "floppy_06".to_string(),
                x: 350.0, y: 350.0, w: 30.0, h: 20.0,
                label_zh: "软盘 DISK_06".to_string(),
                label_en: "Floppy DISK_06".to_string(),
                hover: false,
            },
            // Large areas last
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
        ];

        Self {
            hotspots,
            hovered_id: None,
            offset_x: 0.0,
            offset_y: 0.0,
        }
    }

    /// Update hover state — highlight the smallest matching hotspot
    pub fn update(&mut self, mx: f32, my: f32) {
        let rx = mx - self.offset_x;
        let ry = my - self.offset_y;

        // Reset all hover states
        for hs in &mut self.hotspots {
            hs.hover = false;
        }
        self.hovered_id = None;

        // Find the smallest matching hotspot (most specific)
        let mut best_area = f32::MAX;
        let mut best_idx = None;
        for (i, hs) in self.hotspots.iter().enumerate() {
            if hs.contains(rx, ry) {
                let area = hs.w * hs.h;
                if area < best_area {
                    best_area = area;
                    best_idx = Some(i);
                }
            }
        }
        if let Some(idx) = best_idx {
            self.hotspots[idx].hover = true;
            self.hovered_id = Some(self.hotspots[idx].id.clone());
        }
    }

    /// Check if a hotspot was clicked — returns the smallest matching hotspot
    pub fn clicked(&self, mx: f32, my: f32) -> Option<String> {
        let rx = mx - self.offset_x;
        let ry = my - self.offset_y;

        let mut best_area = f32::MAX;
        let mut best_id = None;
        for hs in &self.hotspots {
            if hs.contains(rx, ry) {
                let area = hs.w * hs.h;
                if area < best_area {
                    best_area = area;
                    best_id = Some(hs.id.clone());
                }
            }
        }
        best_id
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

        // Poster on wall (game company logo)
        draw_rectangle(ox + 620.0, oy + 80.0, 120.0, 160.0, Color::new(0.15, 0.2, 0.35, 1.0));
        draw_rectangle(ox + 630.0, oy + 90.0, 100.0, 80.0, Color::new(0.2, 0.3, 0.5, 1.0));
        // Text on poster
        draw_rectangle(ox + 640.0, oy + 180.0, 80.0, 8.0, Color::new(0.8, 0.8, 0.8, 1.0));
        draw_rectangle(ox + 650.0, oy + 195.0, 60.0, 6.0, Color::new(0.6, 0.6, 0.6, 1.0));

        // Notebook on desk
        draw_rectangle(ox + 500.0, oy + 430.0, 80.0, 50.0, Color::new(0.2, 0.3, 0.5, 1.0));
        draw_rectangle(ox + 505.0, oy + 435.0, 70.0, 40.0, Color::new(0.9, 0.9, 0.85, 1.0));
        // Lines on notebook
        for i in 0..4 {
            draw_rectangle(ox + 510.0, oy + 442.0 + i as f32 * 8.0, 60.0, 1.0, Color::new(0.7, 0.8, 0.9, 1.0));
        }

        // Floppy box on desk
        draw_rectangle(ox + 820.0, oy + 430.0, 60.0, 40.0, Color::new(0.3, 0.3, 0.35, 1.0));
        draw_rectangle(ox + 825.0, oy + 435.0, 50.0, 15.0, Color::new(0.2, 0.2, 0.25, 1.0));

        // Cabinet
        draw_rectangle(ox + 950.0, oy + 400.0, 100.0, 200.0, Color::new(0.35, 0.3, 0.25, 1.0));
        // Cabinet doors
        draw_rectangle(ox + 955.0, oy + 405.0, 45.0, 190.0, Color::new(0.4, 0.35, 0.28, 1.0));
        draw_rectangle(ox + 1000.0, oy + 405.0, 45.0, 190.0, Color::new(0.4, 0.35, 0.28, 1.0));
        // Cabinet handles
        draw_rectangle(ox + 995.0, oy + 490.0, 5.0, 20.0, Color::new(0.6, 0.5, 0.3, 1.0));
        draw_rectangle(ox + 1000.0, oy + 490.0, 5.0, 20.0, Color::new(0.6, 0.5, 0.3, 1.0));

        // Floppy disks scattered around the room
        let floppy_color = Color::new(0.1, 0.1, 0.3, 1.0);
        // DISK_03 — on bookshelf bottom
        draw_rectangle(ox + 60.0, oy + 450.0, 30.0, 20.0, floppy_color);
        draw_rectangle(ox + 62.0, oy + 452.0, 26.0, 8.0, Color::new(0.2, 0.2, 0.4, 1.0));
        // DISK_04 — on wall near calendar
        draw_rectangle(ox + 160.0, oy + 200.0, 30.0, 20.0, floppy_color);
        draw_rectangle(ox + 162.0, oy + 202.0, 26.0, 8.0, Color::new(0.2, 0.2, 0.4, 1.0));
        // DISK_05 — near cabinet
        draw_rectangle(ox + 960.0, oy + 550.0, 30.0, 20.0, floppy_color);
        draw_rectangle(ox + 962.0, oy + 552.0, 26.0, 8.0, Color::new(0.2, 0.2, 0.4, 1.0));
        // DISK_06 — on desk
        draw_rectangle(ox + 350.0, oy + 350.0, 30.0, 20.0, floppy_color);
        draw_rectangle(ox + 352.0, oy + 352.0, 26.0, 8.0, Color::new(0.2, 0.2, 0.4, 1.0));

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
