use super::io::settings_path;
use super::types::AppSettings;
use std::fs;
use tauri::{AppHandle, Manager};

pub fn run_health_check(app: &AppHandle) -> Vec<String> {
    let mut messages = Vec::new();
    let path = settings_path(app);

    if path.exists() {
        match fs::read_to_string(&path) {
            Ok(raw) => {
                if serde_json::from_str::<AppSettings>(&raw).is_err() {
                    messages.push("settings file corrupted".into());
                    let bak = path.with_extension("v1.json");
                    if bak.exists() {
                        if let Ok(bak_data) = fs::read_to_string(&bak) {
                            if serde_json::from_str::<AppSettings>(&bak_data).is_ok() {
                                let _ = fs::write(&path, &bak_data);
                                messages.push("restored settings from backup".into());
                            }
                        }
                    }
                } else {
                    messages.push("settings OK".into());
                }
            }
            Err(_) => messages.push("cannot read settings file".into()),
        }
    } else {
        messages.push("no settings file yet (will create defaults)".into());
    }

    if let Ok(data_dir) = app.path().app_data_dir() {
        let disks = sysinfo::Disks::new_with_refreshed_list();
        for disk in disks.list() {
            if data_dir.starts_with(disk.mount_point()) {
                let free_mb = disk.available_space() / 1024 / 1024;
                if free_mb < 500 {
                    messages.push(format!(
                        "low disk space: {}MB free on {}",
                        free_mb,
                        disk.mount_point().to_string_lossy()
                    ));
                }
                break;
            }
        }
    }

    messages
}
