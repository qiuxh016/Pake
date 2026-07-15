use super::types::*;

pub trait ModuleSettings {
    fn validate(&self) -> Vec<String>;
    fn summary(&self) -> String;
    fn stats(&self) -> serde_json::Value;
}

impl ModuleSettings for AdblockSettings {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        for rule in &self.custom_rules {
            let trimmed = rule.trim();
            if trimmed.is_empty() {
                continue;
            }
            if !(trimmed.starts_with("||") || trimmed.starts_with("##") || trimmed.starts_with('/'))
            {
                errors.push(format!("invalid rule format: '{}'", trimmed));
            }
        }
        errors
    }
    fn summary(&self) -> String {
        if self.enabled {
            format!("enabled, {} custom rules", self.custom_rules.len())
        } else {
            "disabled".into()
        }
    }
    fn stats(&self) -> serde_json::Value {
        serde_json::json!({
            "enabled": self.enabled,
            "rule_count": self.custom_rules.len()
        })
    }
}

impl ModuleSettings for CacheSettings {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        if self.max_size_mb < 50 || self.max_size_mb > 1000 {
            errors.push(format!(
                "cache size {}MB out of range (50-1000)",
                self.max_size_mb
            ));
        }
        errors
    }
    fn summary(&self) -> String {
        if self.enabled {
            format!("enabled, {}MB limit", self.max_size_mb)
        } else {
            "disabled".into()
        }
    }
    fn stats(&self) -> serde_json::Value {
        serde_json::json!({
            "enabled": self.enabled,
            "max_size_mb": self.max_size_mb,
            "hit_rate_1h": self.hit_rate_1h
        })
    }
}

impl ModuleSettings for ClipboardSettings {
    fn validate(&self) -> Vec<String> {
        let mut errors = Vec::new();
        let valid = [500, 1000, 2000, 5000];
        if !valid.contains(&self.max_records) {
            errors.push(format!(
                "max records {} not in allowed values: {:?}",
                self.max_records, valid
            ));
        }
        let valid_days = [7, 14, 30, 0];
        if !valid_days.contains(&self.retention_days) {
            errors.push(format!(
                "retention days {} not in allowed values: {:?}",
                self.retention_days, valid_days
            ));
        }
        errors
    }
    fn summary(&self) -> String {
        if self.enabled {
            format!("enabled, max {} records", self.max_records)
        } else {
            "disabled".into()
        }
    }
    fn stats(&self) -> serde_json::Value {
        serde_json::json!({
            "enabled": self.enabled,
            "max_records": self.max_records,
            "retention_days": self.retention_days
        })
    }
}

impl ModuleSettings for GeneralSettings {
    fn validate(&self) -> Vec<String> {
        // Theme and Language are enums now, always valid by construction
        vec![]
    }
    fn summary(&self) -> String {
        format!("theme={:?}, lang={:?}", self.theme, self.language)
    }
    fn stats(&self) -> serde_json::Value {
        serde_json::json!({
            "theme": self.theme,
            "language": self.language
        })
    }
}
