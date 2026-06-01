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
use game::state::{DosState, CommandResult};
use game::task::TaskSystem;
use game::save::SaveManager;
use game::tutorial::{Tutorial, TutorialStep};
use audio::player::AudioPlayer;

/// Game application states
#[derive(Debug, Clone, PartialEq)]
enum AppState {
    PoweredOff,
    Post,
    BiosSetup,
    DosBoot,
    DosCli,
    Room,
    Dialogue,
    /// Showing branch choices (player picks A/B/C)
    BranchChoice { branch_prefix: String, choices: Vec<(String, String)> },
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
    tutorial: Tutorial,
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
            tutorial: Tutorial::new(),
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

        // Function keys (never conflict with DOS typing)
        // F1 = Language toggle (works everywhere except DOS input)
        // F2 = Return to title menu
        // F7 = Demo dialogue
        if is_key_pressed(KeyCode::F2) && self.state != AppState::PoweredOff {
            self.audio.stop();
            self.dialogue.skip();
            self.state = AppState::PoweredOff;
            return;
        }

        match self.state {
            AppState::PoweredOff => {
                self.vga.clear(0, 0);
                self.vga.set_cursor(4, 22);
                self.vga.put_str("x86 BIOS Simulator", 15, 0);
                self.vga.set_cursor(5, 22);
                self.vga.put_str("==================", 8, 0);

                // Menu text: English in VGA, Chinese via CJK overlay (drawn later)
                match self.language {
                    Language::English => {
                        self.vga.set_cursor(8, 25);
                        self.vga.put_str("[1] New Game", 7, 0);
                        self.vga.set_cursor(9, 25);
                        self.vga.put_str("[2] Continue", 7, 0);
                        self.vga.set_cursor(10, 25);
                        self.vga.put_str("[3] Demo Dialogue", 7, 0);
                        self.vga.set_cursor(12, 25);
                        self.vga.put_str("[F1] Language", 8, 0);
                        self.vga.set_cursor(14, 25);
                        self.vga.put_str("v0.1.0", 8, 0);
                    }
                    Language::Chinese => {
                        // Chinese mode: VGA shows only title, menu drawn by CJK overlay
                        // Leave rows 8-14 empty in VGA buffer
                    }
                }

                // F1 = Language toggle (only in menu)
                if is_key_pressed(KeyCode::F1) {
                    self.language = self.language.toggle();
                }

                // Menu input — keyboard or click anywhere on screen
                if is_key_pressed(KeyCode::Key1) || is_mouse_button_pressed(MouseButton::Left) {
                    self.state = AppState::Post;
                    self.post_progress = 0;
                    self.vga.clear(7, 0);
                }
                if is_key_pressed(KeyCode::Key2) {
                    // Continue — load save slot 0
                    if let Some(save) = self.saves.load(0) {
                        self.play_time = save.play_time_secs as f64;
                        self.language = match save.language.as_str() {
                            "English" => Language::English,
                            _ => Language::Chinese,
                        };
                        self.tasks.current_chapter = save.current_chapter;
                        self.state = AppState::DosCli;
                        self.setup_dos_cli();
                    } else {
                        // No save — start new game
                        self.state = AppState::Post;
                        self.post_progress = 0;
                        self.vga.clear(7, 0);
                    }
                }
                if is_key_pressed(KeyCode::Key3) || is_key_pressed(KeyCode::D) {
                    self.dialogue.start_chapter("chapter_1_player_monologue");
                    self.play_current_dialogue_audio();
                    self.state = AppState::Dialogue;
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
                    self.tasks.complete("boot_computer");
                    self.state = AppState::DosCli;
                    self.setup_dos_cli();
                }
                self.post_progress += 1;
            }
            AppState::DosCli => {
                self.handle_dos_input();
                self.update_cursor_blink();
                // F3 = Enter room exploration
                if is_key_pressed(KeyCode::F3) {
                    self.tutorial.on_enter_room();
                    self.state = AppState::Room;
                }
                // F1 = Language toggle
                if is_key_pressed(KeyCode::F1) {
                    self.language = self.language.toggle();
                }
                // F7 = Demo dialogue
                if is_key_pressed(KeyCode::F7) {
                    self.dialogue.start_chapter("chapter_1_player_monologue");
                    self.play_current_dialogue_audio();
                    self.state = AppState::Dialogue;
                }
                // F5 = Quick save
                if is_key_pressed(KeyCode::F5) {
                    use game::save::SaveData;
                    let mut data = SaveData::new(0);
                    data.current_chapter = self.tasks.current_chapter;
                    data.play_time_secs = self.play_time as u64;
                    data.language = format!("{:?}", self.language);
                    data.floppies_collected = self.tasks.floppies_collected.clone();
                    data.knowledge_cards = self.tasks.knowledge_cards.clone();
                    data.dos_current_dir = self.dos.current_dir.clone();
                    match self.saves.save(data) {
                        Ok(()) => {
                            self.vga.newline();
                            self.vga.put_str("[Saved to slot 1]", 10, 0);
                            self.vga.newline();
                            self.dos.print_prompt(&mut self.vga);
                        }
                        Err(e) => {
                            self.vga.newline();
                            self.vga.put_str(&format!("[Save failed: {}]", e), 12, 0);
                            self.vga.newline();
                            self.dos.print_prompt(&mut self.vga);
                        }
                    }
                }
                // F9 = Quick load
                if is_key_pressed(KeyCode::F9) {
                    if let Some(save) = self.saves.load(0) {
                        self.play_time = save.play_time_secs as f64;
                        self.tasks.current_chapter = save.current_chapter;
                        self.tasks.floppies_collected = save.floppies_collected.clone();
                        self.tasks.knowledge_cards = save.knowledge_cards.clone();
                        self.vga.newline();
                        self.vga.put_str("[Loaded from slot 1]", 10, 0);
                        self.vga.newline();
                        self.dos.print_prompt(&mut self.vga);
                    } else {
                        self.vga.newline();
                        self.vga.put_str("[No save in slot 1]", 12, 0);
                        self.vga.newline();
                        self.dos.print_prompt(&mut self.vga);
                    }
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
                                self.tasks.discover("collect_disk_01");
                                self.tasks.collect_floppy("DISK_01");
                                self.tasks.complete("collect_disk_01");
                                self.dos.collect_floppy("DISK_01");
                                self.tutorial.on_find_floppy();
                                // Show a brief message
                                self.vga.clear(7, 0);
                                self.vga.set_cursor(12, 20);
                                self.vga.put_str("Found: DISK_01 floppy disk!", 14, 0);
                                self.vga.newline();
                                self.vga.set_cursor(14, 20);
                                self.vga.put_str("Press any key to continue...", 8, 0);
                                self.state = AppState::DosCli;
                            }
                            "poster" => {
                                // Poster triggers memory about the game studio
                                self.dialogue.start_chapter("chapter_3_documents");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "notebook" => {
                                // Notebook contains grandfather's notes
                                self.tasks.discover("read_readme");
                                self.vga.clear(7, 0);
                                self.vga.set_cursor(8, 10);
                                self.vga.put_str("Found grandfather's notebook!", 14, 0);
                                self.vga.newline();
                                self.vga.set_cursor(10, 10);
                                self.vga.put_str("Notes about BIOS interrupts and disk sectors.", 7, 0);
                                self.vga.newline();
                                self.vga.set_cursor(12, 10);
                                self.vga.put_str("Try: type README.TXT in DOS", 8, 0);
                                self.state = AppState::DosCli;
                            }
                            "floppy_box" => {
                                if !self.tasks.floppies_collected.contains(&"DISK_02".to_string()) {
                                    self.tasks.collect_floppy("DISK_02");
                                    self.dos.collect_floppy("DISK_02");
                                    self.vga.clear(7, 0);
                                    self.vga.set_cursor(12, 20);
                                    self.vga.put_str("Found: DISK_02 floppy disk!", 14, 0);
                                    self.vga.newline();
                                    self.vga.set_cursor(14, 20);
                                    self.vga.put_str("Contains: DISK_02 data files", 7, 0);
                                } else {
                                    self.vga.clear(7, 0);
                                    self.vga.set_cursor(12, 20);
                                    self.vga.put_str("The floppy box is empty now.", 8, 0);
                                }
                                self.state = AppState::DosCli;
                            }
                            "cabinet" => {
                                self.dialogue.start_chapter("chapter_6_recordings");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "calendar" => {
                                // Calendar triggers photo memories (chapter 7)
                                self.dialogue.start_chapter("chapter_7_photos");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "poster" => {
                                // Poster triggers documents about the company (chapter 3)
                                self.dialogue.start_chapter("chapter_3_documents");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "notebook" => {
                                self.tasks.discover("read_readme");
                                self.dialogue.start_chapter("chapter_1_player_monologue");
                                self.play_current_dialogue_audio();
                                self.state = AppState::Dialogue;
                            }
                            "floppy_03" | "floppy_04" | "floppy_05" | "floppy_06" => {
                                let disk_id = match obj_id.as_str() {
                                    "floppy_03" => "DISK_03",
                                    "floppy_04" => "DISK_04",
                                    "floppy_05" => "DISK_05",
                                    "floppy_06" => "DISK_06",
                                    _ => "",
                                };
                                if !self.tasks.floppies_collected.contains(&disk_id.to_string()) {
                                    self.tasks.collect_floppy(disk_id);
                                    self.dos.collect_floppy(disk_id); // Sync with DOS filesystem
                                    self.tutorial.on_find_floppy();
                                    self.vga.clear(7, 0);
                                    self.vga.set_cursor(12, 15);
                                    self.vga.put_str(&format!("Found: {} floppy disk!", disk_id), 14, 0);
                                    self.vga.newline();
                                    self.vga.set_cursor(14, 15);
                                    self.vga.put_str("New files unlocked! Type DIR in DOS to see them.", 7, 0);
                                } else {
                                    self.vga.clear(7, 0);
                                    self.vga.set_cursor(12, 25);
                                    self.vga.put_str("Already collected this disk.", 8, 0);
                                }
                                self.state = AppState::DosCli;
                            }
                            _ => {}
                        }
                    }
                }
                if is_key_pressed(KeyCode::Escape) {
                    self.tutorial.on_return_dos();
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
            AppState::BranchChoice { ref branch_prefix, ref choices } => {
                // Player selects A/B/C with keyboard
                let selection = if is_key_pressed(KeyCode::A) {
                    choices.iter().find(|(id, _)| id.ends_with("_a"))
                } else if is_key_pressed(KeyCode::B) {
                    choices.iter().find(|(id, _)| id.ends_with("_b"))
                } else if is_key_pressed(KeyCode::C) {
                    choices.iter().find(|(id, _)| id.ends_with("_c"))
                } else {
                    None
                };

                if let Some((id, _)) = selection {
                    let id = id.clone();
                    self.dialogue.start_branch(&id);
                    self.play_current_dialogue_audio();
                    self.state = AppState::Dialogue;
                }

                if is_key_pressed(KeyCode::Escape) {
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

        // Show tutorial hint
        if let Some(hint) = self.tutorial.hint(self.language) {
            for line in hint.split('\n') {
                self.vga.put_str(line, 14, 0);
                self.vga.newline();
            }
            self.vga.newline();
            self.tutorial.mark_shown();
        }

        self.dos.print_prompt(&mut self.vga);
        self.vga.cursor_visible = true;
    }

    fn handle_dos_input(&mut self) {
        use game::state::CommandResult;

        while let Some(ch) = get_char_pressed() {
            if ch.is_ascii() && !ch.is_control() {
                self.dos.input_char(ch);
                self.vga.put_char(ch as u8, 7, 0);
            }
        }
        // Execute command on Enter — handle result
        if is_key_pressed(KeyCode::Enter) {
            let result = self.dos.execute(&mut self.vga);
            match result {
                CommandResult::ReadFile { chapter, file } => {
                    self.tasks.discover("read_readme");
                    if file == "README.TXT" {
                        self.tasks.complete("read_readme");
                        self.tutorial.on_read_readme();
                    } else {
                        self.tutorial.on_read_other();
                    }
                    self.vga.newline();
                    self.vga.put_str("[Loading file into memory...]", 8, 0);
                    self.dialogue.start_chapter(&chapter);
                    self.play_current_dialogue_audio();
                    self.state = AppState::Dialogue;
                }
                CommandResult::DiscoverTask(task_id) => {
                    self.tasks.discover(&task_id);
                }
                CommandResult::CompleteTask(task_id) => {
                    self.tasks.complete(&task_id);
                }
                CommandResult::CollectFloppy(disk_id) => {
                    self.tasks.collect_floppy(&disk_id);
                }
                CommandResult::BadEnding => {
                    // TODO: trigger bad ending scene
                }
                CommandResult::ExitToRoom => {
                    self.state = AppState::Room;
                }
                CommandResult::Dir => {
                    self.tutorial.on_dir();
                }
                CommandResult::ShowBranch(prefix) => {
                    let choices = self.dialogue.get_branch_choices(&prefix, self.language);
                    if !choices.is_empty() {
                        self.state = AppState::BranchChoice {
                            branch_prefix: prefix,
                            choices,
                        };
                    }
                }
                CommandResult::None => {}
            }
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
                // Draw tooltip for hovered object — bilingual
                if let Some(ref obj_id) = self.room.hovered_id {
                    let (mx, my) = mouse_position();
                    let label = match (obj_id.as_str(), self.language) {
                        ("monitor", Language::Chinese) => "CRT 显示器 — 点击进入 DOS",
                        ("monitor", Language::English) => "CRT Monitor — Click to enter DOS",
                        ("keyboard", Language::Chinese) => "键盘",
                        ("keyboard", Language::English) => "Keyboard",
                        ("bookshelf", Language::Chinese) => "书架 — 外公的录音带",
                        ("bookshelf", Language::English) => "Bookshelf — Grandpa's tapes",
                        ("window", Language::Chinese) => "窗户 — 张阿姨家",
                        ("window", Language::English) => "Window — Aunt Zhang's house",
                        ("drawer", Language::Chinese) => "抽屉 — 软盘 DISK_01",
                        ("drawer", Language::English) => "Drawer — Floppy DISK_01",
                        ("lamp", Language::Chinese) => "台灯",
                        ("lamp", Language::English) => "Desk Lamp",
                        ("telephone", Language::Chinese) => "电话 — 李德胜来电",
                        ("telephone", Language::English) => "Telephone — Li Desheng's call",
                        ("calendar", Language::Chinese) => "日历 — 1998 年",
                        ("calendar", Language::English) => "Calendar — Year 1998",
                        ("poster", Language::Chinese) => "海报 — 辛巳游戏工作室",
                        ("poster", Language::English) => "Poster — Xinsi Game Studio",
                        ("notebook", Language::Chinese) => "笔记本 — 外公的笔记",
                        ("notebook", Language::English) => "Notebook — Grandpa's notes",
                        ("floppy_box", Language::Chinese) => "软盘盒 — DISK_02",
                        ("floppy_box", Language::English) => "Floppy Box — DISK_02",
                        ("cabinet", Language::Chinese) => "柜子 — 录音带收藏",
                        ("cabinet", Language::English) => "Cabinet — Tape collection",
                        _ => "",
                    };
                    draw_text_ex(label, mx + 10.0, my - 5.0, TextParams {
                        font: Some(&self.font_cjk),
                        font_size: 16,
                        color: Color::new(1.0, 1.0, 0.8, 0.9),
                        ..Default::default()
                    });
                }
            }
            AppState::Dialogue => {
                self.vga_renderer.draw(&self.vga, crt_x, crt_y, CRT_SCALE, false, false);
                self.draw_dialogue_overlay(crt_x, crt_y);
            }
            AppState::BranchChoice { ref choices, .. } => {
                // Draw VGA background
                self.vga_renderer.draw(&self.vga, crt_x, crt_y, CRT_SCALE, false, false);
                // Draw choice overlay
                let box_h = 200.0;
                let box_y = crt_y + (CRT_CONTENT_H - box_h) / 2.0;
                let box_x = crt_x + 100.0;
                let box_w = CRT_CONTENT_W - 200.0;

                draw_rectangle(box_x, box_y, box_w, box_h, Color::new(0.0, 0.0, 0.1, 0.95));
                draw_rectangle_lines(box_x, box_y, box_w, box_h, 2.0, Color::new(0.3, 0.6, 1.0, 1.0));

                let title_params = TextParams {
                    font: Some(&self.font_cjk),
                    font_size: 20,
                    color: Color::new(0.3, 0.8, 1.0, 1.0),
                    ..Default::default()
                };
                let choice_params = TextParams {
                    font: Some(&self.font_cjk),
                    font_size: 18,
                    color: WHITE,
                    ..Default::default()
                };
                let hint_params = TextParams {
                    font: Some(&self.font_cjk),
                    font_size: 14,
                    color: Color::new(0.5, 0.5, 0.5, 1.0),
                    ..Default::default()
                };

                let title = match self.language {
                    Language::Chinese => "做出你的选择：",
                    Language::English => "Make your choice:",
                };
                draw_text_ex(title, box_x + 16.0, box_y + 28.0, title_params);

                for (i, (_, text)) in choices.iter().enumerate() {
                    let letter = (b'A' + i as u8) as char;
                    let label = format!("[{}] {}", letter, text);
                    draw_text_ex(&label, box_x + 16.0, box_y + 60.0 + i as f32 * 30.0, choice_params.clone());
                }

                draw_text_ex("[A/B/C] Select  [Esc] Cancel", box_x + 16.0, box_y + box_h - 12.0, hint_params);
            }
            _ => {
                self.vga_renderer.draw(
                    &self.vga, crt_x, crt_y, CRT_SCALE,
                    self.vga.cursor_visible, self.cursor_blink,
                );
            }
        }

        // CJK overlay for title menu (VGA can't render Chinese)
        if self.state == AppState::PoweredOff && self.language == Language::Chinese {
            let menu_params = TextParams {
                font: Some(&self.font_cjk),
                font_size: 20,
                color: Color::new(1.0, 1.0, 1.0, 1.0),
                ..Default::default()
            };
            let hint_cjk = TextParams {
                font: Some(&self.font_cjk),
                font_size: 16,
                color: Color::new(0.5, 0.5, 0.5, 1.0),
                ..Default::default()
            };
            let mx = crt_x + 25.0 * CHAR_WIDTH * CRT_SCALE;
            let my = crt_y;

            // Draw black rectangles to cover VGA area for menu rows
            let row_h = CHAR_HEIGHT * CRT_SCALE;
            let menu_w = 30.0 * CHAR_WIDTH * CRT_SCALE;
            for row in 8..=14 {
                draw_rectangle(mx, my + row as f32 * row_h, menu_w, row_h, BLACK);
            }

            draw_text_ex("[1] 新游戏",        mx, my + 8.5 * row_h, menu_params.clone());
            draw_text_ex("[2] 继续游戏",      mx, my + 9.5 * row_h, menu_params.clone());
            draw_text_ex("[3] 演示对话",      mx, my + 10.5 * row_h, menu_params.clone());
            draw_text_ex("[F1] 语言切换",     mx, my + 12.5 * row_h, hint_cjk.clone());
            draw_text_ex("按 1-3 或点击开始", mx, my + 14.5 * row_h, hint_cjk);
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

        // HUD hints — bilingual and complete
        let hint_params = TextParams {
            font: Some(&self.font_cjk),
            font_size: 16,
            color: Color::new(0.4, 0.4, 0.4, 1.0),
            ..Default::default()
        };

        // State-specific hints — function keys only (no single-letter shortcuts)
        let hints = match (&self.state, self.language) {
            (AppState::PoweredOff, Language::Chinese) =>
                "[1] 新游戏  [2] 继续  [3] 演示  [F1] 语言".to_string(),
            (AppState::PoweredOff, Language::English) =>
                "[1] New  [2] Continue  [3] Demo  [F1] Language".to_string(),
            (AppState::DosCli, Language::Chinese) =>
                "[F1] 语言  [F3] 房间  [F5] 存档  [F7] 对话  [F9] 读档  [F2] 菜单".to_string(),
            (AppState::DosCli, Language::English) =>
                "[F1] Lang  [F3] Room  [F5] Save  [F7] Dialogue  [F9] Load  [F2] Menu".to_string(),
            (AppState::Room, Language::Chinese) =>
                "点击物体交互  [F1] 语言  [F2] 菜单  [Esc] DOS".to_string(),
            (AppState::Room, Language::English) =>
                "Click to interact  [F1] Lang  [F2] Menu  [Esc] DOS".to_string(),
            (AppState::Dialogue, Language::Chinese) =>
                "[Enter/Space] 继续  [Esc] 跳过  [F1] 语言  [F2] 菜单".to_string(),
            (AppState::Dialogue, Language::English) =>
                "[Enter/Space] Continue  [Esc] Skip  [F1] Lang  [F2] Menu".to_string(),
            _ => "[F1] Language  [F2] Menu".to_string(),
        };
        draw_text_ex(&hints, 10.0, screen_height() - 10.0, hint_params);

        // Show collected items
        if !self.tasks.floppies_collected.is_empty() {
            let items = match self.language {
                Language::Chinese => format!("软盘: {}", self.tasks.floppies_collected.join(", ")),
                Language::English => format!("Floppies: {}", self.tasks.floppies_collected.join(", ")),
            };
            let item_params = TextParams {
                font: Some(&self.font_cjk),
                font_size: 16,
                color: Color::new(0.4, 0.4, 0.4, 1.0),
                ..Default::default()
            };
            draw_text_ex(&items, 10.0, 40.0, item_params);
        }
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

        // Character name — always from current language
        let char_name = self.dialogue.current_character(self.language);
        draw_text_ex(&char_name, box_x + 16.0, box_y + 24.0, name_params);

        // Dialogue text — always from current language (ignore typewriter for now)
        let display = self.dialogue.current_text(self.language);

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
