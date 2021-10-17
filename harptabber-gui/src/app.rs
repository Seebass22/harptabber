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
    keyboard_layout: Vec<Vec<String>>,

    should_display_notes: bool,
    key: String,
    notes_in_order: Vec<String>,
    duplicated_notes: Vec<String>,
}

impl Default for GUIApp {
    fn default() -> Self {
        let (notes, duplicated) = harptabber::tuning_to_notes_in_order("richter");
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
            keyboard_layout: harptabber::get_tabkeyboard_layout("richter"),

            should_display_notes: false,
            key: "C".to_owned(),
            notes_in_order: notes,
            duplicated_notes: duplicated,
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

        ui.add_space(20.0);

        self.tuning_selector(ui, true);
        self.tuning_selector(ui, false);
    }

    fn tuning_selector(&mut self, ui: &mut egui::Ui, is_input: bool) {
        let mut tuning;
        let label_name;
        if is_input {
            label_name = "input tuning";
            tuning = self.input_tuning.clone();
        } else {
            label_name = "output tuning";
            tuning = self.output_tuning.clone();
        }

        egui::ComboBox::from_label(label_name)
            .selected_text(&mut tuning)
            .width(150.0)
            .show_ui(ui, |ui| {
                if ui
                    .selectable_value(&mut tuning, "richter".to_string(), "richter")
                    .changed()
                    || ui
                        .selectable_value(&mut tuning, "paddy richter".to_string(), "paddy richter")
                        .changed()
                    || ui
                        .selectable_value(&mut tuning, "natural minor".to_string(), "natural minor")
                        .changed()
                    || ui
                        .selectable_value(
                            &mut tuning,
                            "harmonic minor".to_string(),
                            "harmonic minor",
                        )
                        .changed()
                    || ui
                        .selectable_value(&mut tuning, "wilde tuning".to_string(), "wilde tuning")
                        .changed()
                    || ui
                        .selectable_value(
                            &mut tuning,
                            "wilde minor tuning".to_string(),
                            "wilde minor tuning",
                        )
                        .changed()
                    || ui
                        .selectable_value(&mut tuning, "pentaharp".to_string(), "pentaharp")
                        .changed()
                    || ui
                        .selectable_value(&mut tuning, "powerbender".to_string(), "powerbender")
                        .changed()
                    || ui
                        .selectable_value(&mut tuning, "powerdraw".to_string(), "powerdraw")
                        .changed()
                    || ui
                        .selectable_value(&mut tuning, "melody maker".to_string(), "melody maker")
                        .changed()
                    || ui
                        .selectable_value(&mut tuning, "easy 3rd".to_string(), "easy 3rd")
                        .changed()
                    || ui
                        .selectable_value(
                            &mut tuning,
                            "4 hole richter".to_string(),
                            "4 hole richter",
                        )
                        .changed()
                    || ui
                        .selectable_value(
                            &mut tuning,
                            "5 hole richter".to_string(),
                            "5 hole richter",
                        )
                        .changed()
                {
                    if is_input {
                        self.keyboard_layout = harptabber::get_tabkeyboard_layout(&tuning);
                        self.input_tuning = tuning;

                        let (notes, duplicated) =
                            harptabber::tuning_to_notes_in_order(&self.input_tuning);
                        self.notes_in_order = notes;
                        self.duplicated_notes = duplicated;
                    } else {
                        self.output_tuning = tuning;
                    }
                    self.transpose();
                }
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
                ui.checkbox(&mut self.should_display_notes, "display as notes");
                ui.add_space(258.0);
                if ui
                    .add(egui::Button::new("return").text_style(egui::TextStyle::Monospace))
                    .clicked()
                {
                    self.input_text.push('\n');
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

            let rows = self.keyboard_layout.clone();

            for (i, row) in rows.iter().enumerate() {
                ui.horizontal(|ui| {
                    for hole in row {
                        if hole.is_empty() {
                            ui.add(
                                egui::Button::new("     ")
                                    .text_style(egui::TextStyle::Monospace)
                                    .enabled(false),
                            );
                        } else {
                            let display_note;
                            if self.should_display_notes {
                                display_note = harptabber::tab_to_note(
                                    hole,
                                    &self.key,
                                    &self.notes_in_order,
                                    &self.duplicated_notes,
                                );
                            } else {
                                display_note =
                                    harptabber::change_tab_style_single(hole, self.style);
                            }

                            let hole = harptabber::change_tab_style_single(hole, self.style);

                            let text = format!("{:width$}", &display_note, width = 5);
                            if ui
                                .add(
                                    egui::Button::new(text.as_str())
                                        .text_style(egui::TextStyle::Monospace),
                                )
                                .clicked()
                            {
                                self.input_text.push_str(hole.as_str());
                                self.input_text.push(' ');
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
