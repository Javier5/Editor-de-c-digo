use eframe::egui;
use std::process::Command;
use std::fs;

struct EditorApp {
    buffer: String,
    active_panel: String,
    is_panel_open: bool,
}

impl Default for EditorApp {
    fn default() -> Self {
        Self {
            buffer: String::new(),
            active_panel: "Explorer".to_string(),
            is_panel_open: true,
        }
    }
}

impl eframe::App for EditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::SidePanel::left("activity_bar").resizable(false).default_width(45.0).show(ctx, |ui| {
            ui.add_space(10.0);
            if ui.selectable_label(self.active_panel == "Explorer", "📂").clicked() { self.active_panel = "Explorer".to_string(); self.is_panel_open = true; }
            ui.add_space(10.0);
            if ui.selectable_label(self.active_panel == "Git", "🌿").clicked() { self.active_panel = "Git".to_string(); self.is_panel_open = true; }
        });

        if self.is_panel_open {
            egui::SidePanel::left("content_panel").default_width(180.0).show(ctx, |ui| {
                ui.heading(&self.active_panel);
                ui.separator();
                if self.active_panel == "Explorer" {
                    if let Ok(entries) = fs::read_dir(".") {
                        for entry in entries.flatten() {
                            ui.label(format!("📄 {}", entry.file_name().to_string_lossy()));
                        }
                    }
                } else if self.active_panel == "Git" {
                    if ui.button("⬆ Push").clicked() { Command::new("git").args(["push"]).spawn().ok(); }
                    if ui.button("⬇ Pull").clicked() { Command::new("git").args(["pull"]).spawn().ok(); }
                }
            });
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(egui::TextEdit::multiline(&mut self.buffer).desired_width(f32::INFINITY));
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    eframe::run_native("Editor Profesional", eframe::NativeOptions::default(), Box::new(|_| Ok(Box::new(EditorApp::default()))))
}
