use sysinfo;
use crate::ide_utils;

pub fn scan_processes() -> Vec<String> {
    let system = sysinfo::System::new_all();
    let ides = ide_utils::load_jetbrains_ides();
    let mut found_jetbrains_ides: Vec<String> = Vec::new();
    system.processes().iter().for_each(|(pid, process)| {
        if let Some(name) = process.name().to_str() {
            let ide = ide_utils::is_jetbrains_ide(name, &ides);
            if ide.0 && !found_jetbrains_ides.contains(&ide.1) {
                found_jetbrains_ides.push(ide.1);
                println!("✓ {} is a JetBrains IDE", name);
            } else {
                println!("✗ {} is not a JetBrains IDE", name);
            }
        }
    });
    if found_jetbrains_ides.is_empty() {
        return vec!["No JetBrains IDEs found".to_string()];
    }
    found_jetbrains_ides
} 