use std::{collections::HashMap, path::PathBuf};
use sysinfo;
use crate::ide_utils;

pub fn scan_processes() -> HashMap<String, PathBuf> {
    let system = sysinfo::System::new_all();
    let ides = ide_utils::load_jetbrains_ides();
    let mut found_jetbrains_ides: HashMap<String, PathBuf> = HashMap::new();
    system.processes().iter().for_each(|(pid, process)| {
        if let Some(name) = process.name().to_str() {
            let ide = ide_utils::is_jetbrains_ide(name, &ides);
            if ide.0 && !found_jetbrains_ides.contains_key(&ide.1) {
                if let Some(exe) = process.exe() {
                    found_jetbrains_ides.insert(ide.1, exe.to_path_buf());
                }
            }
        }
    });
    found_jetbrains_ides
} 