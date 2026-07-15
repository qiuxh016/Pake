use super::commands::validate_settings;
use super::traits::ModuleSettings;
use super::types::*;

#[test]
fn default_settings_has_sane_values() {
    let s = AppSettings::default();
    assert!(s.adblock.enabled);
    assert!(s.adblock.custom_rules.is_empty());
    assert!(s.cache.enabled);
    assert_eq!(s.cache.max_size_mb, 200);
    assert_eq!(s.cache.hit_rate_1h, 0.0);
    assert!(s.clipboard.enabled);
    assert_eq!(s.clipboard.max_records, 2000);
    assert_eq!(s.clipboard.retention_days, 30);
    assert!(s.clipboard.ignore_short);
    assert_eq!(s.general.theme, Theme::Light);
    assert_eq!(s.general.language, Language::Zh);
}

#[test]
fn settings_json_roundtrip() {
    let s = AppSettings::default();
    let json = serde_json::to_string(&s).expect("serialize");
    let s2: AppSettings = serde_json::from_str(&json).expect("deserialize");
    assert_eq!(s2.adblock.enabled, s.adblock.enabled);
    assert_eq!(s2.cache.max_size_mb, s.cache.max_size_mb);
    assert_eq!(s2.clipboard.max_records, s.clipboard.max_records);
    assert_eq!(s2.general.theme, s.general.theme);
}

#[test]
fn settings_parse_partial_json() {
    let json = r#"{"adblock":{"enabled":true,"custom_rules":["rule1"]}}"#;
    let s: AppSettings = serde_json::from_str(json).expect("parse partial");
    assert!(s.adblock.enabled);
    assert_eq!(s.adblock.custom_rules.len(), 1);
    assert_eq!(s.cache.max_size_mb, 200);
    assert_eq!(s.clipboard.retention_days, 30);
}

#[test]
fn settings_parse_empty_json() {
    let s: AppSettings = serde_json::from_str("{}").expect("parse empty");
    assert!(s.adblock.enabled);
    assert_eq!(s.cache.max_size_mb, 200);
}

#[test]
fn adblock_rules_serialize_correctly() {
    let s = AppSettings {
        adblock: AdblockSettings {
            enabled: true,
            custom_rules: vec!["||example.com^".into(), "##.ad".into()],
        },
        ..Default::default()
    };
    let json = serde_json::to_string(&s).unwrap();
    assert!(json.contains("||example.com^"));
    assert!(json.contains("##.ad"));
}

#[test]
fn theme_serializes_to_lowercase() {
    let s = AppSettings::default();
    let json = serde_json::to_string(&s).unwrap();
    assert!(json.contains("\"light\""));

    let with_theme = GeneralSettings {
        theme: Theme::Dark,
        language: Language::En,
    };
    let json2 = serde_json::to_string(&with_theme).unwrap();
    assert!(json2.contains("\"dark\""));
    assert!(json2.contains("\"en\""));
}

#[test]
fn theme_deserializes_from_lowercase() {
    let json = r#"{"theme":"dark","language":"en"}"#;
    let g: GeneralSettings = serde_json::from_str(json).unwrap();
    assert_eq!(g.theme, Theme::Dark);
    assert_eq!(g.language, Language::En);
}

#[test]
fn theme_deserializes_unknown_as_default() {
    let json = r#"{"theme":"blue","language":"fr"}"#;
    let g: Result<GeneralSettings, _> = serde_json::from_str(json);
    // Unknown variants should fail
    assert!(g.is_err());
}

// ========== Validation tests ==========

#[test]
fn validate_allows_valid_rules() {
    let s = AdblockSettings {
        enabled: true,
        custom_rules: vec!["||example.com^".into(), "##.ad".into(), "/banner/".into()],
    };
    assert!(s.validate().is_empty());
}

#[test]
fn validate_rejects_bad_rule_format() {
    let s = AdblockSettings {
        enabled: true,
        custom_rules: vec!["bad-rule".into()],
    };
    assert!(!s.validate().is_empty());
}

#[test]
fn validate_skips_empty_rules() {
    let s = AdblockSettings {
        enabled: true,
        custom_rules: vec!["  ".into(), "||ok.com^".into()],
    };
    assert!(s.validate().is_empty());
}

#[test]
fn validate_cache_size_range() {
    let s = CacheSettings {
        max_size_mb: 10,
        ..Default::default()
    };
    assert!(!s.validate().is_empty());
    let s2 = CacheSettings {
        max_size_mb: 2000,
        ..Default::default()
    };
    assert!(!s2.validate().is_empty());
    let s3 = CacheSettings {
        max_size_mb: 200,
        ..Default::default()
    };
    assert!(s3.validate().is_empty());
}

#[test]
fn validate_clipboard_max_records() {
    assert!(!ClipboardSettings {
        max_records: 999,
        ..Default::default()
    }
    .validate()
    .is_empty());
    assert!(ClipboardSettings {
        max_records: 2000,
        ..Default::default()
    }
    .validate()
    .is_empty());
}

#[test]
fn validate_clipboard_retention_days() {
    assert!(!ClipboardSettings {
        retention_days: 99,
        ..Default::default()
    }
    .validate()
    .is_empty());
    assert!(ClipboardSettings {
        retention_days: 0,
        ..Default::default()
    }
    .validate()
    .is_empty());
}

#[test]
fn validate_general_enums_always_pass() {
    let good = GeneralSettings {
        theme: Theme::Dark,
        language: Language::En,
    };
    assert!(good.validate().is_empty());
}

#[test]
fn validate_settings_command_fails_on_bad_input() {
    let s = AppSettings {
        cache: CacheSettings {
            max_size_mb: 9999,
            ..Default::default()
        },
        ..Default::default()
    };
    assert!(validate_settings(s).is_err());
}

#[test]
fn summary_shows_module_state() {
    let ad = AdblockSettings {
        enabled: true,
        custom_rules: vec!["||x.com^".into()],
    };
    assert!(ad.summary().contains("enabled"));
    assert!(ad.summary().contains("1"));

    let cache = CacheSettings {
        enabled: false,
        ..Default::default()
    };
    assert!(cache.summary().contains("disabled"));
}

#[test]
fn stats_returns_json_object() {
    let g = GeneralSettings::default();
    let s = g.stats();
    assert_eq!(s["theme"], "light");
    assert_eq!(s["language"], "zh");
}

#[test]
fn diagnostics_has_non_empty_fields() {
    let s = AppSettings::default();
    let _json = serde_json::to_string(&s).unwrap();
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();

    let os_name = sysinfo::System::name().unwrap_or_default();
    assert!(!os_name.is_empty(), "OS name should not be empty");

    let cpu_count = sys.cpus().len();
    assert!(cpu_count > 0, "Should have at least 1 CPU core");

    let total_mem = sys.total_memory();
    assert!(total_mem > 0, "Should have > 0 total memory");

    let disks = sysinfo::Disks::new_with_refreshed_list();
    assert!(!disks.list().is_empty(), "Should have at least 1 disk");
}
