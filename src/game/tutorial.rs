// Tutorial system — step-by-step new player guidance
//
// Guides the player through the first chapter with contextual hints.
// Each step has a trigger condition and a hint message.

use crate::config::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TutorialStep {
    /// Just booted — welcome message
    Welcome,
    /// Player should type DIR to see files
    TypeDir,
    /// Player should type README.TXT
    TypeReadme,
    /// Player read README — now try other files
    TryOtherFiles,
    /// Player should explore the room
    ExploreRoom,
    /// Player found a floppy — return to DOS
    ReturnToDos,
    /// Tutorial complete
    Done,
}

pub struct Tutorial {
    pub step: TutorialStep,
    pub shown: bool,
}

impl Tutorial {
    pub fn new() -> Self {
        Self {
            step: TutorialStep::Welcome,
            shown: false,
        }
    }

    /// Get the current tutorial hint text
    pub fn hint(&self, lang: Language) -> Option<&'static str> {
        if self.step == TutorialStep::Done {
            return None;
        }
        match (self.step, lang) {
            (TutorialStep::Welcome, Language::Chinese) =>
                Some("欢迎来到外公的阁楼。这台电脑已经等了你 20 年。\n输入 HELP 查看可用命令，输入 DIR 查看文件。"),
            (TutorialStep::Welcome, Language::English) =>
                Some("Welcome to Grandpa's attic. This computer has waited 20 years for you.\nType HELP for commands, type DIR to list files."),
            (TutorialStep::TypeDir, Language::Chinese) =>
                Some("很好！你看到了文件列表。\n输入 TYPE README.TXT 阅读外公留下的第一封信。"),
            (TutorialStep::TypeDir, Language::English) =>
                Some("Good! You can see the file list.\nType TYPE README.TXT to read Grandpa's first message."),
            (TutorialStep::TypeReadme, Language::Chinese) =>
                Some("外公说不要格式化 C 盘……\n试试其他命令：TYPE LETTER.TXT、TYPE DIARY.TXT\n或输入 DEBUG 查看磁盘调试器。"),
            (TutorialStep::TypeReadme, Language::English) =>
                Some("Grandpa says don't format C drive...\nTry other commands: TYPE LETTER.TXT, TYPE DIARY.TXT\nor type DEBUG to access the disk debugger."),
            (TutorialStep::TryOtherFiles, Language::Chinese) =>
                Some("你已经读了几份文件。线索正在拼凑……\n按 F3 进入房间，探索周围的物体。"),
            (TutorialStep::TryOtherFiles, Language::English) =>
                Some("You've read several files. The clues are coming together...\nPress F3 to enter the room and explore objects around you."),
            (TutorialStep::ExploreRoom, Language::Chinese) =>
                Some("房间里的每个物体都可能藏着线索。\n试试点击书架、窗户、电话……\n按 Esc 返回 DOS。"),
            (TutorialStep::ExploreRoom, Language::English) =>
                Some("Every object in the room may hide a clue.\nTry clicking the bookshelf, window, telephone...\nPress Esc to return to DOS."),
            (TutorialStep::ReturnToDos, Language::Chinese) =>
                Some("你找到了软盘！回到 DOS 继续探索。\n输入 DIR 看看有没有新文件。"),
            (TutorialStep::ReturnToDos, Language::English) =>
                Some("You found a floppy disk! Return to DOS to continue.\nType DIR to check for new files."),
            _ => None,
        }
    }

    /// Advance to the next step based on what the player did
    pub fn on_dir(&mut self) {
        if self.step == TutorialStep::Welcome {
            self.step = TutorialStep::TypeDir;
            self.shown = false;
        }
    }

    pub fn on_read_readme(&mut self) {
        if self.step == TutorialStep::TypeDir {
            self.step = TutorialStep::TypeReadme;
            self.shown = false;
        }
    }

    pub fn on_read_other(&mut self) {
        if self.step == TutorialStep::TypeReadme {
            self.step = TutorialStep::TryOtherFiles;
            self.shown = false;
        }
    }

    pub fn on_enter_room(&mut self) {
        if self.step == TutorialStep::TryOtherFiles {
            self.step = TutorialStep::ExploreRoom;
            self.shown = false;
        }
    }

    pub fn on_find_floppy(&mut self) {
        if self.step == TutorialStep::ExploreRoom {
            self.step = TutorialStep::ReturnToDos;
            self.shown = false;
        }
    }

    pub fn on_return_dos(&mut self) {
        if self.step == TutorialStep::ReturnToDos {
            self.step = TutorialStep::Done;
        }
    }

    /// Mark current step as shown (don't repeat)
    pub fn mark_shown(&mut self) {
        self.shown = true;
    }
}
