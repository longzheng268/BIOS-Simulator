// DOS command parser — handles player input at the C:\> prompt
//
// Commands return CommandResult to trigger game events.

use crate::render::vga::VgaBuffer;

/// Result of executing a DOS command — tells the game loop what happened
#[derive(Debug, Clone)]
pub enum CommandResult {
    /// Normal output, no special event
    None,
    /// Player read a story file — trigger dialogue chapter
    ReadFile { chapter: String, file: String },
    /// Player discovered hidden data — unlock task
    DiscoverTask(String),
    /// Player completed a task
    CompleteTask(String),
    /// Player found a floppy disk
    CollectFloppy(String),
    /// Player triggered the bad ending
    BadEnding,
    /// Player wants to return to room
    ExitToRoom,
}

/// Current DOS state
pub struct DosState {
    pub current_dir: String,
    pub command_buffer: String,
    pub history: Vec<String>,
    pub files_read: Vec<String>,
}

impl DosState {
    pub fn new() -> Self {
        Self {
            current_dir: "C:\\".to_string(),
            command_buffer: String::new(),
            history: Vec::new(),
            files_read: Vec::new(),
        }
    }

    /// Process a typed character
    pub fn input_char(&mut self, ch: char) {
        if ch.is_ascii() && !ch.is_control() {
            self.command_buffer.push(ch);
        }
    }

    /// Process backspace
    pub fn backspace(&mut self) {
        self.command_buffer.pop();
    }

    /// Execute the current command buffer — returns result for game loop
    pub fn execute(&mut self, vga: &mut VgaBuffer) -> CommandResult {
        let cmd = self.command_buffer.trim().to_string();
        self.history.push(cmd.clone());
        self.command_buffer.clear();

        if cmd.is_empty() {
            vga.newline();
            self.print_prompt(vga);
            return CommandResult::None;
        }

        vga.newline();
        let result = self.run_command(&cmd, vga);
        vga.newline();
        self.print_prompt(vga);
        result
    }

    /// Print the DOS prompt
    pub fn print_prompt(&self, vga: &mut VgaBuffer) {
        vga.put_str(&format!("{}>", self.current_dir), 7, 0);
    }

    /// Run a DOS command — returns result for game loop
    fn run_command(&mut self, cmd: &str, vga: &mut VgaBuffer) -> CommandResult {
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        let command = parts[0].to_uppercase();
        let args = if parts.len() > 1 { parts[1].trim() } else { "" };

        match command.as_str() {
            "DIR" => { cmd_dir(vga, args); CommandResult::None }
            "TYPE" => cmd_type(vga, args, &mut self.files_read),
            "CD" | "CHDIR" => { cmd_cd(vga, args); CommandResult::None }
            "CLS" => { vga.clear(7, 0); CommandResult::None }
            "HELP" | "?" => { cmd_help(vga); CommandResult::None }
            "VER" => { cmd_ver(vga); CommandResult::None }
            "ECHO" => { cmd_echo(vga, args); CommandResult::None }
            "DATE" => { cmd_date(vga); CommandResult::None }
            "TIME" => { cmd_time(vga); CommandResult::None }
            "VOL" => { cmd_vol(vga); CommandResult::None }
            "MEM" => { cmd_mem(vga); CommandResult::None }
            "FORMAT" => cmd_format(vga, args),
            "DEBUG" => cmd_debug(vga),
            "FDISK" => { cmd_fdisk(vga); CommandResult::DiscoverTask("learn_int13h".to_string()) }
            "TREE" => { cmd_tree(vga); CommandResult::None }
            "EXIT" => { CommandResult::ExitToRoom }
            _ => {
                vga.put_str("Bad command or file name", 7, 0);
                CommandResult::None
            }
        }
    }
}

// ─── DOS Commands ───

fn cmd_dir(vga: &mut VgaBuffer, _args: &str) {
    vga.put_str(" Volume in drive C is GRANDPA", 7, 0);
    vga.newline();
    vga.put_str(" Volume Serial Number is 1998-0615", 7, 0);
    vga.newline();
    vga.put_str(" Directory of C:\\", 7, 0);
    vga.newline();
    vga.newline();

    let files = vec![
        ("README   ", "TXT", "1998", "06-15", "256"),
        ("LETTER   ", "TXT", "1998", "06-15", "1,024"),
        ("DIARY    ", "TXT", "1998", "08-03", "2,048"),
        ("EVIDENCE ", "BIN", "1998", "10-12", "4,096"),
        ("FINAL    ", "TXT", "1999", "01-05", "512"),
        ("TOOLS    ", "EXE", "1998", "06-15", "8,192"),
    ];

    for (name, ext, year, date, size) in files {
        vga.put_str(&format!("{} {}  {} {}  {}", name, ext, date, year, size), 7, 0);
        vga.newline();
    }

    vga.newline();
    vga.put_str("        6 file(s)         16,128 bytes", 7, 0);
    vga.newline();
    vga.put_str("        0 dir(s)      1,457,664 bytes free", 7, 0);
}

fn cmd_type(vga: &mut VgaBuffer, filename: &str, files_read: &mut Vec<String>) -> CommandResult {
    if filename.is_empty() {
        vga.put_str("Required parameter missing", 7, 0);
        return CommandResult::None;
    }

    let upper = filename.to_uppercase();
    let result = match upper.as_str() {
        "README.TXT" | "README" => {
            vga.put_str("To whoever uses this computer:", 7, 0);
            vga.newline();
            vga.put_str("If you can see this, you've booted the old PC.", 7, 0);
            vga.newline();
            vga.put_str("There are important things stored here.", 7, 0);
            vga.newline();
            vga.put_str("Check the files on floppy DISK_01.", 7, 0);
            vga.newline();
            vga.put_str("There is a letter inside.", 7, 0);
            vga.newline();
            vga.put_str("Remember: Do NOT format the C drive.", 14, 0);
            vga.newline();
            vga.put_str("                    -- Wang Zhiyuan", 7, 0);
            vga.newline();
            vga.put_str("                    June 15, 1998", 7, 0);
            CommandResult::ReadFile {
                chapter: "chapter_1_player_monologue".to_string(),
                file: "README.TXT".to_string(),
            }
        }
        "LETTER.TXT" | "LETTER" => {
            vga.put_str("Dear child:", 7, 0);
            vga.newline();
            vga.put_str("If you are reading this, I am gone.", 7, 0);
            vga.newline();
            vga.put_str("There are things I must tell you...", 7, 0);
            vga.newline();
            vga.put_str("This computer hides a secret.", 7, 0);
            vga.newline();
            vga.put_str("Not treasure, but truth.", 7, 0);
            vga.newline();
            vga.put_str("About the company I worked for,", 7, 0);
            vga.newline();
            vga.put_str("about the facts that were covered up.", 7, 0);
            vga.newline();
            vga.put_str("                    -- Grandpa", 7, 0);
            CommandResult::ReadFile {
                chapter: "chapter_2_grandfather_voice".to_string(),
                file: "LETTER.TXT".to_string(),
            }
        }
        "DIARY.TXT" | "DIARY" => {
            vga.put_str("March 15, 1998. Clear.", 7, 0);
            vga.newline();
            vga.put_str("My first day at Xinsi Game Studio.", 7, 0);
            vga.newline();
            vga.put_str("The company is small but the atmosphere is great.", 7, 0);
            vga.newline();
            vga.put_str("Director Li seems like a man of great drive.", 7, 0);
            CommandResult::ReadFile {
                chapter: "chapter_3_documents".to_string(),
                file: "DIARY.TXT".to_string(),
            }
        }
        "EVIDENCE.BIN" | "EVIDENCE" => {
            vga.put_str("[ENCRYPTED DATA - DECRYPTION REQUIRED]", 4, 0);
            vga.newline();
            vga.put_str("Use DEBUG command to access INT 13h debugger.", 8, 0);
            vga.newline();
            vga.put_str("Read sectors 200-210 to decode this file.", 8, 0);
            CommandResult::DiscoverTask("decrypt_evidence".to_string())
        }
        "FINAL.TXT" | "FINAL" => {
            vga.put_str("The investigation is over.", 7, 0);
            vga.newline();
            vga.put_str("Li Desheng found a scapegoat and walked free.", 7, 0);
            vga.newline();
            vga.put_str("I know this evidence will be useful someday.", 7, 0);
            vga.newline();
            vga.put_str("I've encrypted it in a hidden disk partition.", 7, 0);
            vga.newline();
            vga.put_str("If someday my descendants find this...", 7, 0);
            vga.newline();
            vga.put_str("Please let the truth come to light.", 14, 0);
            CommandResult::ReadFile {
                chapter: "chapter_8_endings".to_string(),
                file: "FINAL.TXT".to_string(),
            }
        }
        _ => {
            vga.put_str("File not found", 7, 0);
            CommandResult::None
        }
    };

    // Track which files have been read
    if !files_read.contains(&upper) {
        files_read.push(upper);
    }

    result
}

fn cmd_cd(vga: &mut VgaBuffer, args: &str) {
    if args.is_empty() {
        vga.put_str("C:\\", 7, 0);
    } else {
        vga.put_str("Invalid directory", 7, 0);
    }
}

fn cmd_ver(vga: &mut VgaBuffer) {
    vga.put_str("MS-DOS Version 6.22", 7, 0);
}

fn cmd_echo(vga: &mut VgaBuffer, args: &str) {
    if args.is_empty() {
        vga.put_str("ECHO is on", 7, 0);
    } else {
        vga.put_str(args, 7, 0);
    }
}

fn cmd_date(vga: &mut VgaBuffer) {
    vga.put_str("Current date is Sat 06-15-1998", 7, 0);
}

fn cmd_time(vga: &mut VgaBuffer) {
    vga.put_str("Current time is 03:22:15.00a", 7, 0);
}

fn cmd_vol(vga: &mut VgaBuffer) {
    vga.put_str(" Volume in drive C is GRANDPA", 7, 0);
    vga.newline();
    vga.put_str(" Volume Serial Number is 1998-0615", 7, 0);
}

fn cmd_mem(vga: &mut VgaBuffer) {
    vga.put_str("Memory Type        Total    Used    Free", 7, 0);
    vga.newline();
    vga.put_str("──────────────── ─────── ─────── ───────", 7, 0);
    vga.newline();
    vga.put_str("Conventional       640K    156K    484K", 7, 0);
    vga.newline();
    vga.put_str("Upper              155K     47K    108K", 7, 0);
    vga.newline();
    vga.put_str("Reserved           384K    384K      0K", 7, 0);
    vga.newline();
    vga.put_str("Extended (XMS)  15,360K  2,048K 13,312K", 7, 0);
    vga.newline();
    vga.put_str("──────────────── ─────── ─────── ───────", 7, 0);
    vga.newline();
    vga.put_str("Total memory    16,384K  2,635K 13,759K", 7, 0);
}

fn cmd_format(vga: &mut VgaBuffer, args: &str) -> CommandResult {
    if args.to_uppercase().contains("C:") || args.to_uppercase().contains("C") {
        vga.put_str("WARNING: ALL DATA ON NON-REMOVABLE DISK", 12, 0);
        vga.newline();
        vga.put_str("DRIVE C: WILL BE LOST!", 12, 0);
        vga.newline();
        vga.put_str("Proceed with Format (Y/N)?", 7, 0);
        vga.newline();
        vga.newline();
        vga.put_str("... just kidding. Don't do that.", 8, 0);
        vga.newline();
        vga.put_str("Remember: Do not format C drive.", 14, 0);
        CommandResult::None
    } else {
        vga.put_str("Required parameter missing", 7, 0);
        vga.newline();
        vga.put_str("Usage: FORMAT drive:", 7, 0);
        CommandResult::None
    }
}

fn cmd_debug(vga: &mut VgaBuffer) -> CommandResult {
    vga.put_str("INT 13h Disk Debugger", 14, 0);
    vga.newline();
    vga.put_str("=========================================", 8, 0);
    vga.newline();
    vga.put_str("AX=0201  Read Sector", 7, 0);
    vga.newline();
    vga.put_str("  AH=02  Function: Read", 7, 0);
    vga.newline();
    vga.put_str("  AL=01  Sectors: 1", 7, 0);
    vga.newline();
    vga.put_str("  CH=00  Cylinder: 0", 7, 0);
    vga.newline();
    vga.put_str("  CL=C8  Sector: 200 (0xC8)", 14, 0);
    vga.newline();
    vga.put_str("  DH=00  Head: 0", 7, 0);
    vga.newline();
    vga.put_str("  DL=80  Drive: C:", 7, 0);
    vga.newline();
    vga.newline();
    vga.put_str("Reading sector 200...", 7, 0);
    vga.newline();
    vga.put_str("FOUND: $3,000,000 transfer record", 14, 0);
    vga.newline();
    vga.put_str("Recipient: Li Desheng", 14, 0);
    vga.newline();
    vga.put_str("Memo: 'Technical consulting fee'", 12, 0);
    vga.newline();
    vga.newline();
    vga.put_str("This is evidence of embezzlement.", 7, 0);
    vga.newline();
    vga.put_str("Type EVIDENCE.BIN for full details.", 8, 0);
    CommandResult::CompleteTask("recover_sector_200".to_string())
}

fn cmd_fdisk(vga: &mut VgaBuffer) {
    vga.put_str("Fixed Disk Partition Info", 14, 0);
    vga.newline();
    vga.put_str("═══════════════════════════════════════", 8, 0);
    vga.newline();
    vga.newline();
    vga.put_str("Part  Boot  Type    Start    Size", 7, 0);
    vga.newline();
    vga.put_str("────  ────  ──────  ───────  ────────", 7, 0);
    vga.newline();
    vga.put_str("  1    Yes  FAT16   0x0001   2.0 GB", 7, 0);
    vga.newline();
    vga.put_str("  2    No   Hidden  0x0FA1   6.0 GB", 12, 0);
    vga.newline();
    vga.newline();
    vga.put_str("WARNING: Partition 2 is HIDDEN.", 12, 0);
    vga.newline();
    vga.put_str("Use INT 13h to access hidden sectors.", 8, 0);
}

fn cmd_tree(vga: &mut VgaBuffer) {
    vga.put_str("C:\\.", 7, 0);
    vga.newline();
    vga.put_str("├── README.TXT", 7, 0);
    vga.newline();
    vga.put_str("├── LETTER.TXT", 7, 0);
    vga.newline();
    vga.put_str("├── DIARY.TXT", 7, 0);
    vga.newline();
    vga.put_str("├── EVIDENCE.BIN", 7, 0);
    vga.newline();
    vga.put_str("├── FINAL.TXT", 7, 0);
    vga.newline();
    vga.put_str("├── TOOLS.EXE", 7, 0);
    vga.newline();
    vga.put_str("└── [HIDDEN PARTITION]", 12, 0);
}

fn cmd_color(vga: &mut VgaBuffer, args: &str) {
    if args.is_empty() {
        vga.put_str("Sets the console foreground color.", 7, 0);
        vga.newline();
        vga.put_str("COLOR attr", 7, 0);
        vga.newline();
        vga.put_str("  0=Black  1=Blue   2=Green  3=Cyan", 7, 0);
        vga.newline();
        vga.put_str("  4=Red    5=Purple 6=Brown  7=White", 7, 0);
        vga.newline();
        vga.put_str("  8=Gray   9=LBlue  A=LGreen B=LCyan", 7, 0);
        vga.newline();
        vga.put_str("  C=LRed   D=LPurp  E=Yellow F=BWhite", 7, 0);
    } else {
        vga.put_str("Color changed (demo — not yet implemented)", 8, 0);
    }
}

fn cmd_prompt(vga: &mut VgaBuffer, _args: &str) {
    vga.put_str("PROMPT [Grandpa's Computer] $P$G", 7, 0);
}

fn cmd_exit(vga: &mut VgaBuffer) {
    vga.put_str("Returning to room...", 7, 0);
    // Note: actual state change happens in main.rs
}

fn cmd_help(vga: &mut VgaBuffer) {
    let commands = [
        ("DIR", "Displays a list of files"),
        ("TYPE", "Displays the contents of a file"),
        ("CD", "Displays or changes the current directory"),
        ("CLS", "Clears the screen"),
        ("VER", "Displays the DOS version"),
        ("ECHO", "Displays a message"),
        ("DATE", "Displays the date"),
        ("TIME", "Displays the time"),
        ("VOL", "Displays the volume label"),
        ("MEM", "Displays memory usage"),
        ("FORMAT", "Formats a disk (don't do it!)"),
        ("DEBUG", "INT 13h disk debugger"),
        ("FDISK", "Display partition info"),
        ("TREE", "Display directory tree"),
        ("COLOR", "Set console color"),
        ("HELP", "Displays this help"),
    ];

    vga.put_str("For more information, type HELP command", 7, 0);
    vga.newline();
    for (cmd, desc) in commands {
        vga.put_str(&format!("{:10} {}", cmd, desc), 7, 0);
        vga.newline();
    }
}
