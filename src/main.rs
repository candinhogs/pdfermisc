use eframe::egui;
use std::path::PathBuf;
use pdf_oxide::editor::DocumentEditor;
use pdf_oxide::editor::EditableDocument;

struct Pdfermisc {
    files_path: Vec::<PathBuf>,
}

impl Default for Pdfermisc {
    fn default() -> Self {
        Self {
            files_path: Vec::new(),
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
                if ui.button("Merge Listed PDF").clicked() {
                    if self.files_path.len() >= 2 {
                        match DocumentEditor::open(&self.files_path[0]) {
                            Ok(mut editor) => {
                                let mut success = true;

                                for file in &self.files_path[1..] {
                                    if let Err(err) = editor.merge_from(file) {
                                        eprintln!("Error merging {:?}: {:?}", file, err);
                                        success = false;
                                    }
                                }

                                if success {
                                    match editor.save("merged.pdf") {
                                        Ok(_) => println!("Success on 'merged.pdf'!"),
                                        Err(err) => eprintln!("Save error: {:?}", err),
                                    }
                                }
                            }
                            Err(err) => eprintln!("Open error: {:?}", err),
                        }
                    }
                }
                if ui.button("Rotate Selected PDF").clicked() {
                    match DocumentEditor::open(&self.files_path[0]) {
                        Ok(mut editor) => {
                            let mut success = true;
                            if let Err(err) = editor.rotate_all_pages(180) {
                                eprintln!("Error rotating pages {:?}: {:?}", &self.files_path[0], err);
                                success = false;
                            }
                            if success {
                                match editor.save("flipped.pdf") {
                                    Ok(_) => println!("Success on 'flipped.pdf'!"),
                                    Err(err) => eprintln!("Save error {:?}", err),
                                }
                            }
                        }
                        Err(err) => eprintln!("Open error: {:?}", err),
                    }
                }
            });
        });

        egui::CentralPanel::default().show(ui, |ui| {
            ui.style_mut().override_text_style = Some(egui::TextStyle::Heading);
            ui.heading("Pdfermisc");

            if ui.button("Add PDF").clicked() {
                if let Some(path) = rfd::FileDialog::new()
                    .add_filter("PDF", &["pdf"])
                    .pick_file()
                {
                    self.files_path.push(path);
                }
            }
            for file in &self.files_path {
                if let Some(name) = file.file_name() {
                    ui.label(name.to_string_lossy());
                }
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
