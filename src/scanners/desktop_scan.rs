use walkdir::WalkDir;
use std::fs;
use std::collections::HashMap;
use crate::ide_utils::{self, is_jetbrains_ide};

pub fn scan_desktop(dir: &str) -> HashMap<String, String> {
    let mut ides = HashMap::new();
    let ide_list = ide_utils::load_jetbrains_ides();
    
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        if entry.path().extension().and_then(|s| s.to_str()) == Some("desktop") {
            if let Ok(contents) = fs::read_to_string(entry.path()) {
                let file_name = entry.file_name().to_str().unwrap_or("");
                let data = is_jetbrains_ide(file_name, &ide_list);
                
                if data.0 {
                    if let Some(exec) = contents.lines()
                        .find(|line| line.starts_with("Exec="))
                        .map(|line| line.trim_start_matches("Exec=").to_string()) {
                        ides.insert(data.1, exec);
                    }
                }
            }
        }
    }
    ides
}
