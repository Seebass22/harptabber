use eframe::{egui, epi};

pub struct GUIApp {
    input_text: String,
    output_text: String,
    semitone_shift: i32,
}

impl Default for GUIApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            input_text: "".to_owned(),
            output_text: "".to_owned(),
            semitone_shift: 0,
        }
    }
}

impl epi::App for GUIApp {
    fn name(&self) -> &str {
        "harmonica tab transposer"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let Self {
            input_text,
            output_text,
            semitone_shift,
        } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("side_panel").show(ctx, |ui| {
            ui.heading("input");

            ui.text_edit_multiline(input_text);

            ui.add(egui::Slider::new(semitone_shift, -24..=24).text("semitone shift"));
            ui.horizontal(|ui| {
                if ui.button("octave down").clicked() {
                    *semitone_shift -= 12;
                }
                if ui.button("octave up").clicked() {
                    *semitone_shift += 12;
                }
                if ui.button("reset").clicked() {
                    *semitone_shift = 0;
                }
            });

            if ui.button("go").clicked() {
                *output_text =
                    harptabber::transpose_tabs(input_text.clone(), *semitone_shift, true);
            }

            ui.with_layout(egui::Layout::bottom_up(egui::Align::Center), |ui| {
                ui.add(
                    egui::Hyperlink::new("https://github.com/Seebass22/harptabber")
                        .text("Source code"),
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("output");
            ui.text_edit_multiline(output_text);
            egui::warn_if_debug_build(ui);
        });
    }
}
