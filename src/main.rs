use eframe::egui;
use pdf_oxide::editor::DocumentEditor;
use pdf_oxide::editor::EditableDocument;

struct Pdfermisc {
    files_path: Vec::<String>,
}

impl Default for Pdfermisc {
    fn default() -> Self {
        Self {
            files_path: Vec::<String>::new(),
        }
    }
}

impl eframe::App for Pdfermisc {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::Panel::left("menu_panel").show(ui, |ui| {
            egui::MenuBar::new().ui(ui, |ui| {
                ui.menu_button("Options", |ui| {
                    if ui.button("Quit").clicked() {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
            });
            ui.horizontal(|ui| {
                // For some reason, some files turn to criptic text.
                // explore more about this.
                if ui.button("Merge Listed PDF").clicked() {
                    if !self.files_path.is_empty() {
                        match DocumentEditor::open(&self.files_path[0]) {
                            Ok(mut editor) => {
                                let mut success = true;

                                for file in &self.files_path[1..] {
                                    if let Err(err) = editor.merge_from(file) {
                                        eprintln!("Error {}: {:?}", file, err);
                                        success = false;
                                        break;
                                    }
                                }

                                if success {
                                    if let Err(err) = editor.save("merged.pdf") {
                                        eprintln!("Error: {:?}", err);
                                    } else {
                                        println!("Success on 'merged.pdf'!");
                                    }
                                }
                            }
                            Err(err) => eprintln!("Error: {:?}", err),
                        }
                    }
                }
                if ui.button("Rotate Selected PDF").clicked() {
                    todo!();
                }
            });
        });
        
        egui::CentralPanel::default().show(ui, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.heading("Pdfermisc");

            if ui.button("Add PDF").clicked() {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    if let Some(file) = path.to_str() {
                        self.files_path.push(file.to_owned());
                    }
                }
            }
            // Probably not the best option;
            for file in &self.files_path {
                ui.label(file);
            }
        });
    }
}

fn main() -> eframe::Result {
    eframe::run_native(
        "pdfermisc",
        eframe::NativeOptions::default(),
        Box::new(|_cc| Ok(Box::new(Pdfermisc::default()))),
    )
}
