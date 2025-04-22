use std::collections::HashMap;
use directories::UserDirs;
use sysinfo;
use eframe::egui;

mod scanners;
mod ide_utils;

struct MyApp {
    found_ides: Vec<String>,
    selected_ide: String,
    backup_vmoptions: bool,
    home_dir: String,
    desktop_scan: bool,
    ide_paths: HashMap<String, String>,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            desktop_scan: false,
            backup_vmoptions: false,
            found_ides: Vec::new(),
            selected_ide: String::new(),
            home_dir: match directories::UserDirs::new() {
                Some(dirs) => dirs.home_dir().to_string_lossy().to_string(),
                None => "".to_string(),
            },
            ide_paths: HashMap::new(),
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
                        if is_selected {
                           println!("{}", &self.ide_paths[ide]);
                        }
                    }
                });

                
                ui.separator();
                ui.add_space(ui.available_height() - 100.0);
                ui.heading("Tools");
                
                ui.horizontal_centered(|ui| {
                    ui.vertical(|ui| {
                        if ui.button("Scan Now").clicked() {
                            if !self.desktop_scan {
                                self.ide_paths = scanners::process_scan::scan_processes();
                                self.found_ides = self.ide_paths.keys().cloned().collect();
                            } else {
                                self.ide_paths = scanners::desktop_scan::scan_desktop(format!("{}/.local/share/applications",self.home_dir.as_str()).as_str());
                                self.found_ides = self.ide_paths.keys().cloned().collect();
                            }
                            
                            if !self.found_ides.is_empty() && self.selected_ide.is_empty() {
                                self.selected_ide = self.found_ides[0].clone();
                            }
                        }
                        ui.checkbox(&mut self.desktop_scan, "Scan Desktop");
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

