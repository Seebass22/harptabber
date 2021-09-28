use eframe::{egui, epi};
use harptabber::Style;

pub struct GUIApp {
    input_text: String,
    output_text: String,
    semitone_shift: i32,
    from_position: u32,
    to_position: u32,
    style: Style,
}

impl Default for GUIApp {
    fn default() -> Self {
        Self {
            input_text: "".to_owned(),
            output_text: "".to_owned(),
            semitone_shift: 0,
            from_position: 1,
            to_position: 1,
            style: Style::Default,
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
            from_position,
            to_position,
            style,
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

        egui::SidePanel::left("side_panel")
            .default_width(330.0)
            .show(ctx, |ui| {
                ui.heading("input");

                ui.add(egui::TextEdit::multiline(input_text).desired_width(300.0));

                if ui
                    .add(egui::Slider::new(semitone_shift, -24..=24).text("semitone shift"))
                    .changed()
                {
                    *to_position =
                        harptabber::semitones_to_position(*from_position, *semitone_shift);
                }

                ui.horizontal(|ui| {
                    if ui.button("octave down").clicked() {
                        *semitone_shift -= 12;
                    }
                    if ui.button("octave up").clicked() {
                        *semitone_shift += 12;
                    }
                    if ui.button("reset").clicked() {
                        *semitone_shift = 0;
                        *to_position =
                            harptabber::semitones_to_position(*from_position, *semitone_shift);
                    }
                });

                if ui
                    .add(egui::Slider::new(from_position, 1..=12).text("starting position"))
                    .changed()
                {
                    *semitone_shift = harptabber::positions_to_semitones(
                        *from_position as i32,
                        *to_position as i32,
                        0,
                    );
                }
                if ui
                    .add(egui::Slider::new(to_position, 1..=12).text("target position"))
                    .changed()
                {
                    *semitone_shift = harptabber::positions_to_semitones(
                        *from_position as i32,
                        *to_position as i32,
                        0,
                    );
                }
                ui.vertical(|ui| {
                    ui.label("tab style");
                    ui.horizontal(|ui| {
                        ui.selectable_value(style, Style::Default, "default");
                        ui.selectable_value(style, Style::B, "alternative");
                        ui.selectable_value(style, Style::Harpsurgery, "Harpsurgery");
                    });
                });

                if ui.button("go").clicked() {
                    let style = *style;
                    *output_text = harptabber::transpose_tabs(
                        input_text.clone(),
                        *semitone_shift,
                        true,
                        style,
                    );
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
            ui.add(egui::TextEdit::multiline(output_text).desired_width(300.0));
            egui::warn_if_debug_build(ui);
        });
    }
}
