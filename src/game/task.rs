// Task/quest system — tracks player progress
//
// Tasks are triggered by game events (clicking objects, reading files, etc.)
// and track the player's progress through the story.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Task state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskState {
    Hidden,      // Not yet discovered
    Discovered,  // Player has found the trigger
    Active,      // Currently being worked on
    Completed,   // Finished
}

/// A single task/quest
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: String,
    pub title_zh: String,
    pub title_en: String,
    pub description_zh: String,
    pub description_en: String,
    pub state: TaskState,
    pub chapter: u8,       // Which chapter this task belongs to
    pub is_main: bool,     // Main story vs side quest
}

/// Task system managing all tasks
#[derive(Debug, Serialize, Deserialize)]
pub struct TaskSystem {
    pub tasks: HashMap<String, Task>,
    pub current_chapter: u8,
    pub floppies_collected: Vec<String>,
    pub knowledge_cards: Vec<String>,
    pub tapes_listened: Vec<String>,
    pub photos_viewed: Vec<String>,
}

impl TaskSystem {
    pub fn new() -> Self {
        let mut system = Self {
            tasks: HashMap::new(),
            current_chapter: 1,
            floppies_collected: Vec::new(),
            knowledge_cards: Vec::new(),
            tapes_listened: Vec::new(),
            photos_viewed: Vec::new(),
        };
        system.init_tasks();
        system
    }

    fn init_tasks(&mut self) {
        // Chapter 1 tasks
        self.add_task(Task {
            id: "boot_computer".to_string(),
            title_zh: "启动电脑".to_string(),
            title_en: "Boot the Computer".to_string(),
            description_zh: "找到外公的旧电脑，按下电源键".to_string(),
            description_en: "Find Grandpa's old computer and press the power button".to_string(),
            state: TaskState::Active,
            chapter: 1,
            is_main: true,
        });

        self.add_task(Task {
            id: "read_readme".to_string(),
            title_zh: "阅读 README".to_string(),
            title_en: "Read the README".to_string(),
            description_zh: "在 DOS 中执行 type README.TXT".to_string(),
            description_en: "Execute type README.TXT in DOS".to_string(),
            state: TaskState::Hidden,
            chapter: 1,
            is_main: true,
        });

        self.add_task(Task {
            id: "collect_disk_01".to_string(),
            title_zh: "找到第一张软盘".to_string(),
            title_en: "Find the First Floppy".to_string(),
            description_zh: "在房间中找到 DISK_01 软盘".to_string(),
            description_en: "Find DISK_01 floppy disk in the room".to_string(),
            state: TaskState::Hidden,
            chapter: 1,
            is_main: true,
        });

        // Chapter 2 tasks
        self.add_task(Task {
            id: "learn_int13h".to_string(),
            title_zh: "学习 INT 13h".to_string(),
            title_en: "Learn INT 13h".to_string(),
            description_zh: "使用调试器学习磁盘中断调用".to_string(),
            description_en: "Use the debugger to learn disk interrupt calls".to_string(),
            state: TaskState::Hidden,
            chapter: 2,
            is_main: true,
        });

        self.add_task(Task {
            id: "recover_sector_200".to_string(),
            title_zh: "恢复扇区 200".to_string(),
            title_en: "Recover Sector 200".to_string(),
            description_zh: "使用 INT 13h 读取硬盘扇区 200 的隐藏数据".to_string(),
            description_en: "Use INT 13h to read hidden data from hard disk sector 200".to_string(),
            state: TaskState::Hidden,
            chapter: 2,
            is_main: true,
        });

        // Chapter 3 tasks
        self.add_task(Task {
            id: "repair_mbr".to_string(),
            title_zh: "修复 MBR".to_string(),
            title_en: "Repair the MBR".to_string(),
            description_zh: "修复硬盘的主引导记录".to_string(),
            description_en: "Repair the hard disk's Master Boot Record".to_string(),
            state: TaskState::Hidden,
            chapter: 3,
            is_main: true,
        });

        self.add_task(Task {
            id: "decrypt_evidence".to_string(),
            title_zh: "解密证据".to_string(),
            title_en: "Decrypt the Evidence".to_string(),
            description_zh: "解密 EVIDENCE.BIN 文件中的证据".to_string(),
            description_en: "Decrypt the evidence in EVIDENCE.BIN".to_string(),
            state: TaskState::Hidden,
            chapter: 3,
            is_main: true,
        });

        // Chapter 4 tasks
        self.add_task(Task {
            id: "find_truth".to_string(),
            title_zh: "找到真相".to_string(),
            title_en: "Find the Truth".to_string(),
            description_zh: "揭开外公隐藏的真相".to_string(),
            description_en: "Uncover the truth Grandpa hid".to_string(),
            state: TaskState::Hidden,
            chapter: 4,
            is_main: true,
        });

        // Side quests
        self.add_task(Task {
            id: "visit_aunt_zhang".to_string(),
            title_zh: "拜访张阿姨".to_string(),
            title_en: "Visit Aunt Zhang".to_string(),
            description_zh: "通过窗户与邻居张阿姨交谈".to_string(),
            description_en: "Talk to neighbor Aunt Zhang through the window".to_string(),
            state: TaskState::Hidden,
            chapter: 1,
            is_main: false,
        });

        self.add_task(Task {
            id: "listen_all_tapes".to_string(),
            title_zh: "听完所有录音带".to_string(),
            title_en: "Listen to All Tapes".to_string(),
            description_zh: "找到并听完外公留下的 6 盘录音带".to_string(),
            description_en: "Find and listen to all 6 of Grandpa's tapes".to_string(),
            state: TaskState::Hidden,
            chapter: 2,
            is_main: false,
        });
    }

    fn add_task(&mut self, task: Task) {
        self.tasks.insert(task.id.clone(), task);
    }

    /// Mark a task as discovered (player found the trigger)
    pub fn discover(&mut self, task_id: &str) {
        if let Some(task) = self.tasks.get_mut(task_id) {
            if task.state == TaskState::Hidden {
                task.state = TaskState::Discovered;
            }
        }
    }

    /// Mark a task as active (player started working on it)
    pub fn activate(&mut self, task_id: &str) {
        if let Some(task) = self.tasks.get_mut(task_id) {
            if task.state == TaskState::Discovered || task.state == TaskState::Hidden {
                task.state = TaskState::Active;
            }
        }
    }

    /// Complete a task
    pub fn complete(&mut self, task_id: &str) {
        if let Some(task) = self.tasks.get_mut(task_id) {
            task.state = TaskState::Completed;
        }
    }

    /// Check if a task is completed
    pub fn is_completed(&self, task_id: &str) -> bool {
        self.tasks.get(task_id)
            .map(|t| t.state == TaskState::Completed)
            .unwrap_or(false)
    }

    /// Collect a floppy disk
    pub fn collect_floppy(&mut self, disk_id: &str) {
        if !self.floppies_collected.contains(&disk_id.to_string()) {
            self.floppies_collected.push(disk_id.to_string());
        }
    }

    /// Unlock a knowledge card
    pub fn unlock_card(&mut self, card_id: &str) {
        if !self.knowledge_cards.contains(&card_id.to_string()) {
            self.knowledge_cards.push(card_id.to_string());
        }
    }

    /// Get active tasks for the current chapter
    pub fn active_tasks(&self) -> Vec<&Task> {
        self.tasks.values()
            .filter(|t| t.state == TaskState::Active && t.chapter <= self.current_chapter)
            .collect()
    }

    /// Get completion stats
    pub fn stats(&self) -> (usize, usize, usize, usize) {
        let total = self.tasks.len();
        let completed = self.tasks.values().filter(|t| t.state == TaskState::Completed).count();
        (total, completed, self.floppies_collected.len(), self.knowledge_cards.len())
    }
}
