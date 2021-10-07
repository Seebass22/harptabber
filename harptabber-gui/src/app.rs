use eframe::{egui, epi};
use harptabber::Style;

pub struct GUIApp {
    input_text: String,
    output_text: String,
    semitone_shift: i32,
    from_position: u32,
    to_position: u32,
    style: Style,
    style_example: String,
    input_tuning: String,
    output_tuning: String,
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
            style_example: "-2 -2'' -3 4 -4 5 5o 6".to_owned(),
            input_tuning: "richter".to_owned(),
            output_tuning: "richter".to_owned(),
        }
    }
}

impl epi::App for GUIApp {
    fn name(&self) -> &str {
        "harmonica tab transposer"
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let style: egui::Style = (*ui.ctx().style()).clone();
                let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
                if let Some(visuals) = new_visuals {
                    ui.ctx().set_visuals(visuals);
                }
                egui::menu::menu(ui, "File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
                egui::menu::menu(ui, "About", |ui| {
                    ui.add(
                        egui::Hyperlink::new("https://github.com/Seebass22/harptabber")
                            .text("Source code"),
                    );
                });
            });
        });

        egui::SidePanel::left("side_panel")
            .default_width(330.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.leftpanel(ui);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("output");
                ui.add(egui::TextEdit::multiline(&mut self.output_text).desired_width(300.0));
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

impl GUIApp {
    fn transpose(&mut self) {
        self.output_text = harptabber::transpose_tabs(
            self.input_text.clone(),
            self.semitone_shift,
            true,
            self.style,
            &self.input_tuning,
            &self.output_tuning,
        );
    }

    fn leftpanel(&mut self, ui: &mut egui::Ui) {
        ui.heading("input");

        if ui
            .add(egui::TextEdit::multiline(&mut self.input_text).desired_width(600.0))
            .changed()
        {
            self.transpose();
        };

        if ui
            .add(egui::Slider::new(&mut self.semitone_shift, -24..=24).text("semitone shift"))
            .changed()
        {
            self.to_position =
                harptabber::semitones_to_position(self.from_position, self.semitone_shift);
            self.transpose();
        }

        ui.horizontal(|ui| {
            if ui.button("octave down").clicked() {
                self.semitone_shift -= 12;
                self.transpose();
            }
            if ui.button("octave up").clicked() {
                self.semitone_shift += 12;
                self.transpose();
            }
            if ui.button("reset").clicked() {
                self.semitone_shift = 0;
                self.to_position =
                    harptabber::semitones_to_position(self.from_position, self.semitone_shift);
                self.transpose();
            }
        });

        if ui
            .add(egui::Slider::new(&mut self.from_position, 1..=12).text("starting position"))
            .changed()
        {
            self.semitone_shift = harptabber::positions_to_semitones(
                self.from_position as i32,
                self.to_position as i32,
                0,
            );
            self.transpose();
        }
        if ui
            .add(egui::Slider::new(&mut self.to_position, 1..=12).text("target position"))
            .changed()
        {
            self.semitone_shift = harptabber::positions_to_semitones(
                self.from_position as i32,
                self.to_position as i32,
                0,
            );
            self.transpose();
        }

        ui.add_space(20.0);

        ui.collapsing("tab keyboard", |ui| {
            self.tabkeyboard(ui);
        });

        ui.add_space(20.0);

        ui.vertical(|ui| {
            ui.label("tab style");
            ui.horizontal(|ui| {
                self.tab_style_selector(ui);
            });
            ui.add(
                egui::TextEdit::singleline(&mut self.style_example)
                    .desired_width(350.0)
                    .enabled(false),
            );
        });
    }

    fn tab_style_selector(&mut self, ui: &mut egui::Ui) {
        if ui
            .selectable_value(&mut self.style, Style::Default, "default")
            .clicked()
        {
            self.style_example = String::from("-2 -2'' -3 4 -4 5 5o 6");
        }
        if ui
            .selectable_value(&mut self.style, Style::BBends, "b-bends")
            .clicked()
        {
            self.style_example = String::from("-2 -2bb -3 4 -4 5 5o 6");
        }
        if ui
            .selectable_value(&mut self.style, Style::DrawDefault, "draw-default")
            .clicked()
        {
            self.style_example = String::from("2 2'' 3 +4 4 +5 +5o +6");
        }
        if ui
            .selectable_value(&mut self.style, Style::Plus, "plus/minus")
            .clicked()
        {
            self.style_example = String::from("-2 -2'' -3 +4 -4 +5 +5o +6");
        }
        if ui
            .selectable_value(&mut self.style, Style::Harpsurgery, "harpsurgery")
            .clicked()
        {
            self.style_example = String::from("2D 2D'' 3D 4B 4D 5B 5B# 6B");
        }
    }

    fn tabkeyboard(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.add_space(373.0);
                if ui
                    .add(egui::Button::new("return").text_style(egui::TextStyle::Monospace))
                    .clicked()
                {
                    self.input_text.push_str("\n");
                }

                if ui
                    .add(egui::Button::new("backspace").text_style(egui::TextStyle::Monospace))
                    .clicked()
                {
                    self.input_text.pop();
                    let mut last = self.input_text.chars().last();
                    while last.is_some() && last.unwrap() != ' ' {
                        self.input_text.pop();
                        last = self.input_text.chars().last();
                    }
                    self.transpose();
                }
            });

            let rows = vec![
                ["", "", "", "", "", "", "", "", "", "10''"],
                ["1o", "", "", "4o", "5o", "6o", "", "8'", "9'", "10'"],
                ["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
                ["-1", "-2", "-3", "-4", "-5", "-6", "-7", "-8", "-9", "-10"],
                [
                    "-1'", "-2'", "-3'", "-4'", "", "-6'", "-7o", "", "-9o", "-10o",
                ],
                ["", "-2''", "-3''", "", "", "", "", "", "", ""],
                ["", "", "-3'''", "", "", "", "", "", "", ""],
            ];
            for (i, row) in rows.iter().enumerate() {
                ui.horizontal(|ui| {
                    for hole in row {
                        if *hole == "" {
                            ui.add(
                                egui::Button::new("     ")
                                    .text_style(egui::TextStyle::Monospace)
                                    .enabled(false),
                            );
                        } else {
                            let hole = harptabber::change_tab_style_single(hole, self.style);

                            let text = format!("{:width$}", hole, width = 5);
                            if ui
                                .add(
                                    egui::Button::new(text.as_str())
                                        .text_style(egui::TextStyle::Monospace),
                                )
                                .clicked()
                            {
                                self.input_text.push_str(hole.as_str());
                                self.input_text.push_str(" ");
                                self.transpose();
                            }
                        }
                    }
                });
                if i == 2 {
                    ui.horizontal(|ui| {
                        for hole in row {
                            let text = format!("{:width$}", hole, width = 5);
                            ui.add(
                                egui::Button::new(text.as_str())
                                    .text_style(egui::TextStyle::Monospace)
                                    .enabled(false),
                            );
                        }
                    });
                }
            }
        });
    }
}
