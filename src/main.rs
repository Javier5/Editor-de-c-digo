use eframe::egui;

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
        // 1. BARRA LATERAL DE ICONOS (Activity Bar estilo VS Code)
        egui::SidePanel::left("activity_bar")
            .resizable(false)
            .default_width(45.0)
            .show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    ui.add_space(10.0);
                    if ui.selectable_label(self.is_panel_open && self.active_panel == "Explorer", "📂").clicked() {
                        if self.active_panel == "Explorer" {
                            self.is_panel_open = !self.is_panel_open;
                        } else {
                            self.active_panel = "Explorer".to_string();
                            self.is_panel_open = true;
                        }
                    }
                    ui.add_space(10.0);
                    if ui.selectable_label(self.is_panel_open && self.active_panel == "Git", "🌿").clicked() {
                        if self.active_panel == "Git" {
                            self.is_panel_open = !self.is_panel_open;
                        } else {
                            self.active_panel = "Git".to_string();
                            self.is_panel_open = true;
                        }
                    }
                });
            });

        // 2. PANEL CONTENEDOR DESPLEGABLE (Muestra el explorador o Git)
        if self.is_panel_open {
            egui::SidePanel::left("content_panel")
                .resizable(true)
                .default_width(180.0)
                .show(ctx, |ui| {
                    ui.heading(&self.active_panel);
                    ui.separator();
                    if self.active_panel == "Explorer" {
                        ui.label("📁 src");
                        ui.label("  📄 main.rs");
                        ui.label("  📄 language.rs");
                    } else if self.active_panel == "Git" {
                        ui.label("Branch: main");
                        if ui.button("Sync Changes (Push)").clicked() {
                            // Lógica de git posterior
                        }
                    }
                });
        }

        // 3. ÁREA DE TEXTO PRINCIPAL
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.add(
                    egui::TextEdit::multiline(&mut self.buffer)
                        .font(egui::TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(30)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY),
                );
            });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Editor Profesional",
        options,
        Box::new(|_cc| Ok(Box::new(EditorApp::default()))),
    )
}
