#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)]

use std::f32::consts::E;
use std::ffi::OsStr;
use std::fs;
use serde_json::Value;
// it's an example
use sysinfo;
use eframe::egui;

struct MyApp {

    search_text: String,
    found_ides: Vec<String>,
    selected_ide: String,
    auto_scan: bool,
    scan_interval: f32,
    backup_vmoptions: bool,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            backup_vmoptions: false,
            search_text: String::new(),
            found_ides: Vec::new(),
            selected_ide: String::new(),
            auto_scan: false,
            scan_interval: 5.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("Found JetBrains IDEs");
                
                egui::ScrollArea::vertical().show(ui, |ui| {
                    for ide in &self.found_ides {
                        let is_selected = self.selected_ide == *ide;
                        ui.horizontal(|ui| {
                            if ui.add_sized(ui.available_size(), egui::SelectableLabel::new(is_selected, ide)).clicked() {
                                self.selected_ide = ide.clone();
                            }
                        });
                    }
                });

                
                ui.separator();
                ui.add_space(ui.available_height() - 100.0);
                ui.heading("Tools");
                
                ui.horizontal_centered(|ui| {
                    ui.vertical(|ui| {
                        if ui.button("Scan Now").clicked() {
                            self.found_ides = test();
                            if !self.found_ides.is_empty() && self.selected_ide.is_empty() {
                                self.selected_ide = self.found_ides[0].clone();
                            }
                        }
                    
                    });
                    
                    
                    
                    ui.vertical(|ui| {
                        
                        if !self.selected_ide.is_empty() {
                            ui.heading("IDE Controls");
                            if ui.button("Patch Selected IDE").clicked() {
                                println!("Launching {}", self.selected_ide);
                            }
                            if ui.checkbox(&mut self.backup_vmoptions, "Backup VM Options").clicked() {
                                println!("Showing info for {}", self.selected_ide);
                            }
                        }
                    });
                });

             

            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([480.0, 360.0]),
        ..Default::default()
    };

    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

fn load_jetbrains_ides() -> Vec<String> {
    match fs::read_to_string("names.json") {
        Ok(contents) => {
            match serde_json::from_str::<Value>(&contents) {
                Ok(json) => {
                    json["jetbrains_ides"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                }
                Err(e) => {
                    eprintln!("Error parsing JSON: {}", e);
                    Vec::new()
                }
            }
        }
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            Vec::new()
        }
    }
}

fn is_jetbrains_ide(name: &str, ides: &[String]) -> (bool,String) {
    for ide in ides {
        if ide.to_lowercase() == name.to_lowercase() {
            return (true,ide.to_string())
        }
    }
    return (false,String::new())
}

fn test() -> Vec<String> {
    let system = sysinfo::System::new_all();
    let ides = load_jetbrains_ides();
    let mut found_jetbrains_ides: Vec<String> = Vec::new();
    system.processes().iter().for_each(|(pid, process)| {
        if let Some(name) = process.name().to_str() {
            let ide = is_jetbrains_ide(name, &ides);
            if ide.0 && !found_jetbrains_ides.contains(&ide.1) {
                found_jetbrains_ides.push(ide.1);
                println!("✓ {} is a JetBrains IDE", name);
            } else {
                println!("✗ {} is not a JetBrains IDE", name);
            }
        }
    });
    found_jetbrains_ides
}