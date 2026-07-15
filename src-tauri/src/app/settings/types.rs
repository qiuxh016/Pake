use serde::{Deserialize, Serialize};

// ========== Theme & Language enums ==========

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    #[serde(alias = "zh-CN")]
    Zh,
    En,
}

pub fn default_theme() -> Theme {
    Theme::Light
}
pub fn default_language() -> Language {
    Language::Zh
}

// ========== Settings data types ==========

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppSettings {
    #[serde(default)]
    pub adblock: AdblockSettings,
    #[serde(default)]
    pub cache: CacheSettings,
    #[serde(default)]
    pub clipboard: ClipboardSettings,
    #[serde(default)]
    pub general: GeneralSettings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AdblockSettings {
    #[serde(default = "bool_true")]
    pub enabled: bool,
    #[serde(default)]
    pub custom_rules: Vec<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CacheSettings {
    #[serde(default = "bool_true")]
    pub enabled: bool,
    #[serde(default = "default_cache_size")]
    pub max_size_mb: u32,
    #[serde(default)]
    pub hit_rate_1h: f64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ClipboardSettings {
    #[serde(default = "bool_true")]
    pub enabled: bool,
    #[serde(default = "default_max_records")]
    pub max_records: u32,
    #[serde(default = "default_retention_days")]
    pub retention_days: u32,
    #[serde(default = "bool_true")]
    pub ignore_short: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralSettings {
    #[serde(default = "default_theme")]
    pub theme: Theme,
    #[serde(default = "default_language")]
    pub language: Language,
}

fn bool_true() -> bool {
    true
}
fn default_cache_size() -> u32 {
    200
}
fn default_max_records() -> u32 {
    2000
}
fn default_retention_days() -> u32 {
    30
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            adblock: AdblockSettings {
                enabled: true,
                custom_rules: vec![],
            },
            cache: CacheSettings {
                enabled: true,
                max_size_mb: 200,
                hit_rate_1h: 0.0,
            },
            clipboard: ClipboardSettings {
                enabled: true,
                max_records: 2000,
                retention_days: 30,
                ignore_short: true,
            },
            general: GeneralSettings {
                theme: Theme::Light,
                language: Language::Zh,
            },
        }
    }
}

impl Default for AdblockSettings {
    fn default() -> Self {
        AppSettings::default().adblock
    }
}
impl Default for CacheSettings {
    fn default() -> Self {
        AppSettings::default().cache
    }
}
impl Default for ClipboardSettings {
    fn default() -> Self {
        AppSettings::default().clipboard
    }
}
impl Default for GeneralSettings {
    fn default() -> Self {
        AppSettings::default().general
    }
}
