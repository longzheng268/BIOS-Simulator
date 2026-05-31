/// Internationalization module — bilingual Chinese/English support
///
/// Loads text from game_script.json and provides runtime language switching.

use crate::config::Language;

/// A bilingual text string
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LocalizedText {
    pub zh: String,
    pub en: String,
}

impl LocalizedText {
    pub fn get(&self, lang: Language) -> &str {
        match lang {
            Language::Chinese => &self.zh,
            Language::English => &self.en,
        }
    }
}

/// Character name localization
pub fn character_name(id: &str, lang: Language) -> &str {
    match (id, lang) {
        ("player", Language::Chinese) => "林远",
        ("player", Language::English) => "Lin Yuan",
        ("grandfather", Language::Chinese) => "外公",
        ("grandfather", Language::English) => "Grandpa",
        ("aunt_zhang", Language::Chinese) => "张阿姨",
        ("aunt_zhang", Language::English) => "Aunt Zhang",
        ("li_desheng", Language::Chinese) => "李德胜",
        ("li_desheng", Language::English) => "Li Desheng",
        ("narrator", Language::Chinese) => "旁白",
        ("narrator", Language::English) => "Narrator",
        ("computer", Language::Chinese) => "电脑",
        ("computer", Language::English) => "Computer",
        ("env_monologue", Language::Chinese) => "林远",
        ("env_monologue", Language::English) => "Lin Yuan",
        _ => id,
    }
}
