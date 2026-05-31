// DOS command parser — handles player input at the C:\> prompt
//
// Supported commands: dir, type, cd, cls, help, ver, echo, date, time
// Integrates with the virtual filesystem to display file contents.

use crate::render::vga::VgaBuffer;

/// Current DOS state
pub struct DosState {
    pub current_dir: String,
    pub command_buffer: String,
    pub history: Vec<String>,
}

impl DosState {
    pub fn new() -> Self {
        Self {
            current_dir: "C:\\".to_string(),
            command_buffer: String::new(),
            history: Vec::new(),
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

    /// Execute the current command buffer
    pub fn execute(&mut self, vga: &mut VgaBuffer) {
        let cmd = self.command_buffer.trim().to_string();
        self.history.push(cmd.clone());
        self.command_buffer.clear();

        if cmd.is_empty() {
            vga.newline();
            self.print_prompt(vga);
            return;
        }

        vga.newline();
        self.run_command(&cmd, vga);
        vga.newline();
        self.print_prompt(vga);
    }

    /// Print the DOS prompt
    pub fn print_prompt(&self, vga: &mut VgaBuffer) {
        vga.put_str(&format!("{}>", self.current_dir), 7, 0);
    }

    /// Run a DOS command
    fn run_command(&self, cmd: &str, vga: &mut VgaBuffer) {
        let parts: Vec<&str> = cmd.splitn(2, ' ').collect();
        let command = parts[0].to_uppercase();
        let args = if parts.len() > 1 { parts[1].trim() } else { "" };

        match command.as_str() {
            "DIR" => cmd_dir(vga, args),
            "TYPE" => cmd_type(vga, args),
            "CD" | "CHDIR" => cmd_cd(vga, args),
            "CLS" => vga.clear(7, 0),
            "HELP" | "?" => cmd_help(vga),
            "VER" => cmd_ver(vga),
            "ECHO" => cmd_echo(vga, args),
            "DATE" => cmd_date(vga),
            "TIME" => cmd_time(vga),
            "VOL" => cmd_vol(vga),
            "MEM" => cmd_mem(vga),
            "FORMAT" => cmd_format(vga, args),
            "DEBUG" => cmd_debug(vga),
            "FDISK" => cmd_fdisk(vga),
            "TREE" => cmd_tree(vga),
            "COLOR" => cmd_color(vga, args),
            "PROMPT" => cmd_prompt(vga, args),
            "EXIT" => cmd_exit(vga),
            _ => {
                vga.put_str(&format!("Bad command or file name"), 7, 0);
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

fn cmd_type(vga: &mut VgaBuffer, filename: &str) {
    if filename.is_empty() {
        vga.put_str("Required parameter missing", 7, 0);
        return;
    }

    let upper = filename.to_uppercase();
    match upper.as_str() {
        "README.TXT" | "README" => {
            vga.put_str("致使用这台电脑的人：", 7, 0);
            vga.newline();
            vga.put_str("如果你能看到这段话，说明你已经成功启动了这台老电脑。", 7, 0);
            vga.newline();
            vga.put_str("这台电脑里保存着一些重要的东西。", 7, 0);
            vga.newline();
            vga.put_str("请仔细查看 DISK_01 软盘中的文件。", 7, 0);
            vga.newline();
            vga.put_str("里面有一封信。", 7, 0);
            vga.newline();
            vga.put_str("记得：不要格式化 C 盘。", 7, 0);
            vga.newline();
            vga.put_str("                    —— 王志远", 7, 0);
            vga.newline();
            vga.put_str("                    1998 年 6 月 15 日", 7, 0);
        }
        "LETTER.TXT" | "LETTER" => {
            vga.put_str("亲爱的孩子：", 7, 0);
            vga.newline();
            vga.put_str("如果你在读这封信，说明我不在了。", 7, 0);
            vga.newline();
            vga.put_str("有些事情，我必须告诉你……", 7, 0);
            vga.newline();
            vga.put_str("这台电脑里藏着一个秘密。", 7, 0);
            vga.newline();
            vga.put_str("不是什么宝藏，而是一段真相。", 7, 0);
            vga.newline();
            vga.put_str("关于我工作过的那家公司，", 7, 0);
            vga.newline();
            vga.put_str("关于那些被掩盖的事实。", 7, 0);
            vga.newline();
            vga.put_str("                    —— 外公", 7, 0);
        }
        "DIARY.TXT" | "DIARY" => {
            vga.put_str("1998年3月15日，晴。", 7, 0);
            vga.newline();
            vga.put_str("今天是我在新思游戏工作室上班的第一天。", 7, 0);
            vga.newline();
            vga.put_str("公司虽然不大，但氛围很好。", 7, 0);
            vga.newline();
            vga.put_str("李总看起来是个很有魄力的人。", 7, 0);
        }
        "EVIDENCE.BIN" | "EVIDENCE" => {
            vga.put_str("[ENCRYPTED DATA - DECRYPTION REQUIRED]", 4, 0);
            vga.newline();
            vga.put_str("Use INT 13h debugger to decode sectors 200-210", 8, 0);
        }
        "FINAL.TXT" | "FINAL" => {
            vga.put_str("调查结束了。", 7, 0);
            vga.newline();
            vga.put_str("李德胜找了个替罪羊，自己全身而退。", 7, 0);
            vga.newline();
            vga.put_str("我知道这些证据总有一天会派上用场。", 7, 0);
            vga.newline();
            vga.put_str("我把它们加密保存在磁盘的隐藏分区里。", 7, 0);
            vga.newline();
            vga.put_str("如果有一天我的后人能找到这些……", 7, 0);
            vga.newline();
            vga.put_str("请让真相大白于天下。", 7, 0);
        }
        _ => {
            vga.put_str("File not found", 7, 0);
        }
    }
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

fn cmd_format(vga: &mut VgaBuffer, args: &str) {
    if args.to_uppercase().contains("C:") || args.to_uppercase().contains("C") {
        // Bad ending trigger!
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
    } else {
        vga.put_str("Required parameter missing", 7, 0);
        vga.newline();
        vga.put_str("Usage: FORMAT drive:", 7, 0);
    }
}

fn cmd_debug(vga: &mut VgaBuffer) {
    vga.put_str("INT 13h Disk Debugger", 14, 0);
    vga.newline();
    vga.put_str("═══════════════════════════════════════", 8, 0);
    vga.newline();
    vga.put_str("AX=0201  Read Sector", 7, 0);
    vga.newline();
    vga.put_str("  AH=02  Function: Read", 7, 0);
    vga.newline();
    vga.put_str("  AL=01  Sectors: 1", 7, 0);
    vga.newline();
    vga.put_str("  CH=00  Cylinder: 0", 7, 0);
    vga.newline();
    vga.put_str("  CL=01  Sector: 1", 7, 0);
    vga.newline();
    vga.put_str("  DH=00  Head: 0", 7, 0);
    vga.newline();
    vga.put_str("  DL=80  Drive: C:", 7, 0);
    vga.newline();
    vga.newline();
    vga.put_str("Example: Read sector 200 from C:", 8, 0);
    vga.newline();
    vga.put_str("  AX=0201 CH=00 CL=C8 DH=00 DL=80", 14, 0);
    vga.newline();
    vga.newline();
    vga.put_str("Sector 200 (0xC8) contains hidden data.", 7, 0);
    vga.newline();
    vga.put_str("Try: type EVIDENCE.BIN after reading.", 8, 0);
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
