mod config;
mod i18n;
mod core;
mod bios;
mod filesystem;
mod game;
mod render;
mod audio;

use macroquad::prelude::*;
use config::*;
use render::vga::{VgaBuffer, VgaRenderer};
use render::crt::CrtEffect;
use render::room::Room;
use game::dialogue::DialogueEngine;
use game::state::DosState;
use game::task::TaskSystem;
use game::save::SaveManager;
use audio::player::AudioPlayer;

/// Game application states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AppState {
    PoweredOff,
    Post,
    BiosSetup,
    DosBoot,
    DosCli,
    Room,
    Dialogue,
}

struct Game {
    state: AppState,
    vga: VgaBuffer,
    vga_renderer: VgaRenderer,
    crt: Option<CrtEffect>,
    room: Room,
    dos: DosState,
    tasks: TaskSystem,
    saves: SaveManager,
    cursor_blink: bool,
    post_progress: usize,
    play_time: f64,
    language: config::Language,
    dialogue: DialogueEngine,
    audio: AudioPlayer,
    last_time: f64,
    font_cjk: Font,
}

impl Game {
    async fn new() -> Self {
        let mut vga = VgaBuffer::new();
        vga.clear(7, 0);

        let mut dialogue = DialogueEngine::new();
        match dialogue.load("audio_gen/scripts/game_script.json") {
            Ok(()) => println!("Dialogue loaded successfully"),
            Err(e) => eprintln!("Warning: Could not load dialogue: {}", e),
        }

        // Load CJK font for dialogue text (Chinese + English)
        let font_cjk = load_ttf_font("assets/fonts/MiSans-Normal.ttf")
            .await
            .expect("Failed to load MiSans-Normal.ttf");

        // CRT shader — try to create, fall back to direct render if it fails
        let crt = CrtEffect::new(CRT_CONTENT_W as u32, CRT_CONTENT_H as u32).ok();
        if crt.is_some() {
            println!("CRT shader loaded");
        } else {
            println!("CRT shader unavailable, using direct render");
        }

        Self {
            state: AppState::PoweredOff,
            vga,
            vga_renderer: VgaRenderer::new(),
            crt,
            room: Room::new(),
            dos: DosState::new(),
            tasks: TaskSystem::new(),
            saves: SaveManager::new(),
            cursor_blink: true,
            post_progress: 0,
            play_time: 0.0,
            language: config::Language::default(),
            dialogue,
            audio: AudioPlayer::new(),
            last_time: macroquad::time::get_time(),
            font_cjk,
        }
    }

    fn update(&mut self) {
        let now = macroquad::time::get_time();
        let dt = now - self.last_time;
        self.last_time = now;
        self.play_time += dt;

        // Global: toggle language with L key
        if is_key_pressed(KeyCode::L) {
            self.language = self.language.toggle();
            println!("Language toggled to: {:?}", self.language);
        }

        // Global: press D to demo a dialogue chapter
        if is_key_pressed(KeyCode::D) && self.state != AppState::Dialogue {
            println!("Starting dialogue demo...");
            self.dialogue.start_chapter("chapter_1_player_monologue");
            self.play_current_dialogue_audio();
            self.state = AppState::Dialogue;
        }

        // Debug: log mouse clicks in PoweredOff state
        if self.state == AppState::PoweredOff && is_mouse_button_pressed(MouseButton::Left) {
            println!("Mouse clicked in PoweredOff state!");
        }

        match self.state {
            AppState::PoweredOff => {
                self.vga.clear(0, 0);
                self.vga.set_cursor(12, 25);
                self.vga.put_str("[ Click to Power On | D: Demo Dialogue ]", 8, 0);

                if is_mouse_button_pressed(MouseButton::Left) {
                    self.state = AppState::Post;
                    self.post_progress = 0;
                    self.vga.clear(7, 0);
                }
            }
            AppState::Post => {
                self.vga.clear(7, 0);
                self.vga.set_cursor(0, 0);
                self.vga.put_str("PhoenixBIOS 4.0 Release 6.0", 7, 0);
                self.vga.newline();
                self.vga.put_str("Copyright 1985-1998 Phoenix Technologies Ltd.", 7, 0);
                self.vga.newline();
                self.vga.newline();
                let mem_kb = (self.post_progress * 4096).min(65536);
                self.vga.put_str(&format!("Memory Test: {}K OK", mem_kb), 7, 0);
                self.vga.newline();

                if self.post_progress >= 16 {
                    self.vga.newline();
                    self.vga.put_str("Press F2 to enter Setup, F12 for Boot Menu", 8, 0);
                    self.vga.newline();
                    self.vga.put_str("Starting DOS...", 7, 0);
                    self.state = AppState::DosCli;
                    self.setup_dos_cli();
                }
                self.post_progress += 1;
            }
            AppState::DosCli => {
                self.handle_dos_input();
                self.update_cursor_blink();
                // Press R to enter room exploration
                if is_key_pressed(KeyCode::R) {
                    self.state = AppState::Room;
                }
            }
            AppState::Room => {
                let (mx, my) = mouse_position();
                self.room.update(mx, my);
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(obj_id) = self.room.clicked(mx, my) {
                        println!("Clicked: {}", obj_id);
                        match obj_id.as_str() {
                            "monitor" => {
                                self.tasks.discover("boot_computer");
                                self.state = AppState::DosCli;
                                self.setup_dos_cli();
                            }
                            "bookshelf" => {
                                self.tasks.discover("listen_all_tapes");
                                self.dialogue.start_chapter("chapter_2_grandfather_voice");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "telephone" => {
                                self.dialogue.start_chapter("chapter_5_li_desheng");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "window" => {
                                self.tasks.discover("visit_aunt_zhang");
                                self.dialogue.start_chapter("chapter_4_aunt_zhang");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "drawer" => {
                                // Drawer contains a floppy disk
                                self.tasks.discover("collect_disk_01");
                                self.tasks.collect_floppy("DISK_01");
                                self.tasks.complete("collect_disk_01");
                                // Show a brief message
                                self.vga.clear(7, 0);
                                self.vga.set_cursor(12, 20);
                                self.vga.put_str("Found: DISK_01 floppy disk!", 14, 0);
                                self.vga.newline();
                                self.vga.set_cursor(14, 20);
                                self.vga.put_str("Press any key to continue...", 8, 0);
                                self.state = AppState::DosCli;
                            }
                            _ => {}
                        }
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.state = AppState::DosCli;
                    self.setup_dos_cli();
                }
            }
            AppState::Dialogue => {
                self.dialogue.update(dt);
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::Space) {
                    self.dialogue.advance();
                    if self.dialogue.is_active {
                        self.play_current_dialogue_audio();
                    } else {
                        self.audio.stop();
                        self.state = AppState::DosCli;
                        self.setup_dos_cli();
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.dialogue.skip();
                    self.audio.stop();
                    self.state = AppState::DosCli;
                    self.setup_dos_cli();
                }
            }
            _ => {}
        }
    }

    fn setup_dos_cli(&mut self) {
        self.vga.clear(7, 0);
        self.vga.set_cursor(0, 0);
        self.vga.put_str("Microsoft(R) MS-DOS(R) Version 6.22", 7, 0);
        self.vga.newline();
        self.vga.put_str("(C)Copyright Microsoft Corp 1981-1994.", 7, 0);
        self.vga.newline();
        self.vga.newline();
        self.dos.print_prompt(&mut self.vga);
        self.vga.cursor_visible = true;
    }

    fn handle_dos_input(&mut self) {
        // Process typed characters
        while let Some(ch) = get_char_pressed() {
            if ch.is_ascii() && !ch.is_control() {
                self.dos.input_char(ch);
                self.vga.put_char(ch as u8, 7, 0);
            }
        }
        // Execute command on Enter
        if is_key_pressed(KeyCode::Enter) {
            self.dos.execute(&mut self.vga);
        }
        // Backspace
        if is_key_pressed(KeyCode::Backspace) {
            if !self.dos.command_buffer.is_empty() {
                self.dos.backspace();
                if self.vga.cursor_col > 0 {
                    self.vga.cursor_col -= 1;
                    self.vga.put_char(b' ', 7, 0);
                    self.vga.cursor_col -= 1;
                }
            }
        }
    }

    fn update_cursor_blink(&mut self) {
        let now = macroquad::time::get_time();
        self.cursor_blink = ((now * 1000.0) as u64 / CURSOR_BLINK_MS % 2) == 0;
    }

    /// Queue the audio file for the current dialogue segment
    fn play_current_dialogue_audio(&mut self) {
        if let Some(ref chapter_key) = self.dialogue.current_chapter {
            let ck = chapter_key.clone();
            if let Some(ref script) = self.dialogue.script {
                if let Some(chapter) = script.chapters.get(ck.as_str()) {
                    if let Some(segment) = chapter.segments.get(self.dialogue.current_segment_index) {
                        let path = AudioPlayer::dialogue_audio_path(
                            ck.as_str(), &segment.id, self.language,
                        );
                        self.audio.request_play(&path);
                    }
                }
            }
        }
    }

    fn draw(&self) {
        clear_background(Color::new(0.05, 0.05, 0.08, 1.0));

        let crt_x = (screen_width() - CRT_CONTENT_W) / 2.0;
        let crt_y = (screen_height() - CRT_CONTENT_H) / 2.0;

        // CRT monitor frame
        let border = 16.0;
        draw_rectangle(
            crt_x - border, crt_y - border,
            CRT_CONTENT_W + border * 2.0, CRT_CONTENT_H + border * 2.0,
            Color::new(0.15, 0.15, 0.15, 1.0),
        );

        // Draw content based on state
        match self.state {
            AppState::Room => {
                // Draw room scene (full window)
                self.room.draw();
                // Draw tooltip for hovered object
                if let Some(ref obj_id) = self.room.hovered_id {
                    let (mx, my) = mouse_position();
                    let label = match obj_id.as_str() {
                        "monitor" => "CRT Monitor",
                        "keyboard" => "Keyboard",
                        "bookshelf" => "Bookshelf",
                        "window" => "Window",
                        "drawer" => "Drawer",
                        "lamp" => "Desk Lamp",
                        "telephone" => "Telephone",
                        "calendar" => "Calendar",
                        _ => "",
                    };
                    draw_text(label, mx + 10.0, my - 5.0, 16.0, Color::new(1.0, 1.0, 0.8, 0.9));
                }
            }
            AppState::Dialogue => {
                self.vga_renderer.draw(&self.vga, crt_x, crt_y, CRT_SCALE, false, false);
                self.draw_dialogue_overlay(crt_x, crt_y);
            }
            _ => {
                self.vga_renderer.draw(
                    &self.vga, crt_x, crt_y, CRT_SCALE,
                    self.vga.cursor_visible, self.cursor_blink,
                );
            }
        }

        // Apply CRT shader overlay if available
        if let Some(ref _crt) = self.crt {
            // TODO: Apply CRT post-processing effect
            // For now, direct render without CRT overlay
        }

        // Power LED
        let led_color = match self.state {
            AppState::PoweredOff => Color::new(0.2, 0.0, 0.0, 1.0),
            _ => Color::new(0.0, 0.8, 0.0, 1.0),
        };
        draw_circle(crt_x + CRT_CONTENT_W + border - 10.0, crt_y + CRT_CONTENT_H + border - 10.0, 3.0, led_color);

        // Language indicator
        let lang_text = match self.language {
            Language::Chinese => "CN",
            Language::English => "EN",
        };
        draw_text(lang_text, 10.0, 20.0, 20.0, Color::new(0.5, 0.5, 0.5, 1.0));

        // HUD hints (use CJK font)
        let hint_params = TextParams {
            font: Some(&self.font_cjk),
            font_size: 16,
            color: Color::new(0.4, 0.4, 0.4, 1.0),
            ..Default::default()
        };
        draw_text_ex("[L] Language  [D] Demo Dialogue", 10.0, screen_height() - 10.0, hint_params);
    }

    fn draw_dialogue_overlay(&self, crt_x: f32, crt_y: f32) {
        let box_h = 160.0;
        let box_y = crt_y + CRT_CONTENT_H - box_h;
        let box_x = crt_x;
        let box_w = CRT_CONTENT_W;

        // Dark overlay background
        draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.0, 0.0, 0.0, 0.85));
        draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.3, 0.6, 1.0, 1.0));

        // CJK font params
        let name_params = TextParams {
            font: Some(&self.font_cjk),
            font_size: 20,
            color: Color::new(0.3, 0.8, 1.0, 1.0),
            ..Default::default()
        };
        let text_params = TextParams {
            font: Some(&self.font_cjk),
            font_size: 18,
            color: WHITE,
            ..Default::default()
        };
        let hint_params = TextParams {
            font: Some(&self.font_cjk),
            font_size: 16,
            color: Color::new(0.3, 0.6, 1.0, 1.0),
            ..Default::default()
        };

        // Character name
        let char_name = self.dialogue.current_character(self.language);
        draw_text_ex(&char_name, box_x + 16.0, box_y + 24.0, name_params);

        // Dialogue text
        let text = self.dialogue.current_text(self.language);
        let display = if self.dialogue.waiting_for_input {
            text
        } else {
            self.dialogue.display_text.clone()
        };

        // Word-wrap text in the dialogue box
        let max_w = box_w - 32.0;
        let font_size = 18.0;
        let mut line_y = box_y + 48.0;
        let mut line = String::new();
        for word in display.split_inclusive(|c: char| c.is_whitespace()) {
            let test = format!("{}{}", line, word);
            let measured = measure_text(&test, Some(&self.font_cjk), font_size as u16, 1.0);
            if measured.width > max_w && !line.is_empty() {
                draw_text_ex(&line, box_x + 16.0, line_y, text_params.clone());
                line_y += font_size + 4.0;
                line = word.trim_start().to_string();
            } else {
                line = test;
            }
        }
        if !line.trim().is_empty() {
            draw_text_ex(&line, box_x + 16.0, line_y, text_params);
        }

        // "Continue" prompt
        if self.dialogue.waiting_for_input {
            let blink = ((macroquad::time::get_time() * 2.0) as i32 % 2) == 0;
            if blink {
                draw_text_ex(">>", box_x + box_w - 40.0, box_y + box_h - 12.0, hint_params);
            }
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut game = Game::new().await;

    loop {
        game.update();
        game.audio.update().await;
        game.draw();
        next_frame().await;
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: WINDOW_TITLE.to_string(),
        window_width: WINDOW_WIDTH,
        window_height: WINDOW_HEIGHT,
        window_resizable: false,
        ..Default::default()
    }
}
