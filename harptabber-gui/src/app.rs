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

    error_text: String,
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

            error_text: "".to_owned(),
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
                egui::menu::menu(ui, "Help", |ui| {
                    ui.label("- do not use double quotes");
                    ui.label("(use single quotes for");
                    ui.label("bends)");
                    ui.add_space(10.0);
                    ui.label("- if a note is too high");
                    ui.label("or low to be played, or");
                    ui.label("would require bending");
                    ui.label("an overblow, it will");
                    ui.label("appear as X");
                    ui.add_space(10.0);
                    ui.label("- don't forget to set the");
                    ui.label("tab style to the one you");
                    ui.label("use, so the tab can be");
                    ui.label("interpreted correctly");
                    ui.add_space(10.0);
                    ui.label("- everything other a valid");
                    ui.label("note is ignored");
                    ui.add_space(10.0);
                    ui.label("- the tab keyboard can");
                    ui.label("only append to the tab");
                });
                egui::menu::menu(ui, "About", |ui| {
                    ui.label("harptabber-gui");
                    ui.add_space(10.0);
                    ui.label("Copyright © 2021");
                    ui.label("Sebastian Thuemmel");
                    ui.add_space(10.0);
                    ui.add(
                        egui::Hyperlink::new("https://github.com/Seebass22/harptabber")
                            .text("source code"),
                    );
                    ui.add(
                        egui::Hyperlink::new("https://seebass22.itch.io/harmonica-tab-transposer")
                            .text("web version & downloads"),
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

                if !self.error_text.is_empty() {
                    ui.add_space(20.0);

                    ui.label("invalid notes");
                    ui.add_enabled(
                        false,
                        egui::TextEdit::multiline(&mut self.error_text).desired_width(300.0),
                    );
                }
            });
        });
    }
}

impl GUIApp {
    fn transpose(&mut self) {
        let (tabs, errors) = harptabber::transpose_tabs(
            self.input_text.clone(),
            self.semitone_shift,
            true,
            self.style,
            &self.input_tuning,
            &self.output_tuning,
        );
        self.output_text = tabs;
        self.error_text = errors.join(" ");
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

        self.tuning_selector(ui, true);
        self.tuning_selector(ui, false);

        ui.add_space(20.0);

        ui.vertical(|ui| {
            ui.label("tab style");
            ui.horizontal(|ui| {
                self.tab_style_selector(ui);
            });
            ui.add(
                egui::TextEdit::singleline(&mut self.style_example)
                    .desired_width(350.0)
                    .interactive(false),
            );
        });
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
                for tuning_text in [
                    "richter",
                    "paddy richter",
                    "country",
                    "natural minor",
                    "harmonic minor",
                    "wilde tuning",
                    "wilde minor tuning",
                    "pentaharp",
                    "powerbender",
                    "powerdraw",
                    "lucky 13 powerchromatic",
                    "melody maker",
                    "easy 3rd",
                    "4 hole richter",
                    "5 hole richter",
                ]
                .iter()
                {
                    if ui
                        .selectable_value(&mut tuning, tuning_text.to_string(), tuning_text)
                        .changed()
                    {
                        if is_input {
                            self.keyboard_layout = harptabber::get_tabkeyboard_layout(&tuning);
                            self.input_tuning = tuning.clone();

                            let (notes, duplicated) =
                                harptabber::tuning_to_notes_in_order(&self.input_tuning);
                            self.notes_in_order = notes;
                            self.duplicated_notes = duplicated;
                        } else {
                            self.output_tuning = tuning.clone();
                        }
                        self.transpose();
                    }
                }
            });
    }

    fn tab_style_selector(&mut self, ui: &mut egui::Ui) {
        if ui
            .selectable_value(&mut self.style, Style::Default, "default")
            .clicked()
        {
            self.style_example = String::from("-2 -2'' -3 4 -4 5 5o 6");
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::BBends, "b-bends")
            .clicked()
        {
            self.style_example = String::from("-2 -2bb -3 4 -4 5 5o 6");
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::DrawDefault, "draw-default")
            .clicked()
        {
            self.style_example = String::from("2 2'' 3 +4 4 +5 +5o +6");
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::Plus, "plus/minus")
            .clicked()
        {
            self.style_example = String::from("-2 -2'' -3 +4 -4 +5 +5o +6");
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::Harpsurgery, "harpsurgery")
            .clicked()
        {
            self.style_example = String::from("2D 2D'' 3D 4B 4D 5B 5B# 6B");
            self.transpose();
        }
    }

    fn tabkeyboard(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.should_display_notes, "display as notes");
                if self.should_display_notes {
                    egui::ComboBox::from_label("key")
                        .selected_text(&mut self.key)
                        .width(60.0)
                        .show_ui(ui, |ui| {
                            for note in [
                                "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
                            ]
                            .iter()
                            {
                                ui.selectable_value(&mut self.key, note.to_string(), note);
                            }
                        });
                    ui.add_space(155.0);
                } else {
                    ui.add_space(258.0);
                }
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
                    self.backspace();
                    self.transpose();
                }
            });

            let rows = self.keyboard_layout.clone();

            for (i, row) in rows.iter().enumerate() {
                ui.horizontal(|ui| {
                    for hole in row {
                        if hole.is_empty() {
                            ui.add_enabled(
                                false,
                                egui::Button::new("     ")
                                    .text_style(egui::TextStyle::Monospace)
                                    .fill(egui::color::Color32::TRANSPARENT),
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
                            ui.add_enabled(
                                false,
                                egui::Button::new(text.as_str())
                                    .text_style(egui::TextStyle::Monospace),
                            );
                        }
                    });
                }
            }
        });
    }

    fn backspace(&mut self) {
        self.input_text.pop();
        let mut last = self.input_text.chars().last();
        while last.is_some() && last.unwrap() != ' ' {
            self.input_text.pop();
            last = self.input_text.chars().last();
        }
    }
}
