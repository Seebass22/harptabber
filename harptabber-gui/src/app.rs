use eframe::egui;
use eframe::egui::{Button, RichText, Slider, TextEdit, TextStyle, ViewportCommand};
use harptabber::Style;
use std::collections::BTreeMap;

#[cfg(not(target_arch = "wasm32"))]
use rodio::{OutputStream, Sink};

pub struct GUIApp {
    input_text: String,
    output_text: String,
    semitone_shift: i32,
    from_position: u32,
    to_position: u32,
    style: Style,
    style_example: &'static str,
    input_tuning: &'static str,
    output_tuning: &'static str,
    keyboard_layout: Vec<Vec<String>>,
    keyboard_text: String,

    display_as: DisplayOption,
    key: String,
    notes_in_order: Vec<String>,
    duplicated_notes: Vec<String>,
    playable_without_overblows: Vec<(u32, i32)>,
    playable_without_bends: Vec<(u32, i32)>,
    allow_bends: bool,
    keep_errors: bool,

    error_text: String,
    about_open: bool,
    help_open: bool,

    #[cfg(not(target_arch = "wasm32"))]
    audio_context: AudioContext,
    should_play_note: bool,

    scales: &'static BTreeMap<String, Vec<&'static str>>,
    selected_scale: Option<&'static str>,
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum DisplayOption {
    Tabs,
    Degrees,
    Notes,
}

#[cfg(not(target_arch = "wasm32"))]
struct AudioContext {
    _output_stream: rodio::OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
    sink: rodio::Sink,
}

#[cfg(not(target_arch = "wasm32"))]
impl AudioContext {
    fn new() -> Self {
        let (_output_stream, _stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&_stream_handle).unwrap();
        AudioContext {
            _output_stream,
            _stream_handle,
            sink,
        }
    }
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
            style_example: "-2 -2'' -3 4 -4 5 5o 6",
            input_tuning: "richter",
            output_tuning: "richter",
            keyboard_layout: harptabber::get_tabkeyboard_layout("richter"),
            keyboard_text: "".to_owned(),

            display_as: DisplayOption::Tabs,
            key: "C".to_owned(),
            notes_in_order: notes,
            duplicated_notes: duplicated,
            playable_without_overblows: Vec::new(),
            playable_without_bends: Vec::new(),
            allow_bends: true,
            keep_errors: false,

            error_text: "".to_owned(),
            about_open: false,
            help_open: false,

            #[cfg(not(target_arch = "wasm32"))]
            audio_context: AudioContext::new(),
            should_play_note: false,

            scales: harptabber::get_scales(),
            selected_scale: None,
        }
    }
}

impl GUIApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        let mut default = Self::default();
        default.generate_keyboard_text();
        default
    }
}

impl eframe::App for GUIApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let style: egui::Style = (*ui.ctx().style()).clone();
                let new_visuals = style.visuals.light_dark_small_toggle_button(ui);
                if let Some(visuals) = new_visuals {
                    ui.ctx().set_visuals(visuals);
                }

                ui.menu_button("File", |ui| {
                    #[cfg(not(target_arch = "wasm32"))]
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(ViewportCommand::Close);
                    }
                });

                if ui.button("Help").clicked() {
                    self.help_open = true;
                }

                if ui.button("About").clicked() {
                    self.about_open = true;
                }

                self.scale_menu(ui);
            });
        });

        egui::SidePanel::left("side_panel")
            .default_width(550.0)
            .show(ctx, |ui| {
                egui::ScrollArea::vertical().show(ui, |ui| {
                    self.leftpanel(ui);
                });
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.heading("output");

                ui.add(TextEdit::multiline(&mut self.output_text).desired_width(800.0));
                egui::warn_if_debug_build(ui);

                ui.horizontal(|ui| {
                    if ui.button("copy").clicked() {
                        ui.output_mut(|o| o.copied_text = self.output_text.clone());
                    }
                    if ui.checkbox(&mut self.keep_errors, "keep errors").changed() {
                        self.transpose();
                    }
                });

                if !self.error_text.is_empty() {
                    ui.add_space(20.0);

                    ui.label("invalid notes");
                    ui.add_enabled(
                        false,
                        TextEdit::multiline(&mut self.error_text).desired_width(300.0),
                    );
                }

                ui.collapsing("playable positions", |ui| {
                    self.playable_positions_panel(ui);
                });
            });
        });

        self.help_window(ctx);
        self.about_window(ctx);
    }
}

impl GUIApp {
    fn transpose(&mut self) {
        let (tabs, errors) = harptabber::transpose_tabs(
            self.input_text.clone(),
            self.semitone_shift,
            self.keep_errors,
            self.style,
            self.input_tuning,
            self.output_tuning,
        );
        self.output_text = tabs;
        self.error_text = errors.join(" ");

        self.playable_without_overblows = harptabber::get_playable_positions(
            &self.input_text,
            self.from_position,
            self.input_tuning,
            self.output_tuning,
            self.style,
            true,
        );
        self.playable_without_bends = harptabber::get_playable_positions(
            &self.input_text,
            self.from_position,
            self.input_tuning,
            self.output_tuning,
            self.style,
            false,
        );
        self.generate_keyboard_text();
    }

    fn playable_positions_panel(&mut self, ui: &mut egui::Ui) {
        let pairs: &[(u32, i32)] = if self.allow_bends {
            self.playable_without_overblows.as_ref()
        } else {
            self.playable_without_bends.as_ref()
        };

        ui.horizontal(|ui| {
            ui.selectable_value(&mut self.allow_bends, true, "without overblows");
            ui.selectable_value(&mut self.allow_bends, false, "without bends");
        });

        ui.add_enabled(
            false,
            Button::new(
                RichText::new("position, semitone change").text_style(TextStyle::Monospace),
            ),
        );
        for (position, semitones) in pairs.iter() {
            let text = format!(
                "{:width$} {:+width$}",
                harptabber::to_ordinal(*position),
                semitones,
                width = 7
            );

            if ui
                .add(Button::new(
                    RichText::new(text).text_style(TextStyle::Monospace),
                ))
                .clicked()
            {
                self.semitone_shift = *semitones;
                self.to_position = *position;
                self.transpose();
                break;
            }
        }
    }

    fn leftpanel(&mut self, ui: &mut egui::Ui) {
        ui.heading("input");

        let input_field = egui::TextEdit::multiline(&mut self.input_text).desired_width(600.0);
        let tedit_output = input_field.show(ui);
        if tedit_output.response.changed() {
            self.transpose();
        }

        if ui.button("copy").clicked() {
            ui.output_mut(|o| o.copied_text = self.input_text.clone());
        }

        ui.spacing_mut().slider_width = 150.0;
        if ui
            .add(Slider::new(&mut self.semitone_shift, -24..=24).text("semitone shift"))
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
            .add(Slider::new(&mut self.from_position, 1..=12).text("starting position"))
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
            .add(Slider::new(&mut self.to_position, 1..=12).text("target position"))
            .changed()
        {
            self.semitone_shift = harptabber::positions_to_semitones(
                self.from_position as i32,
                self.to_position as i32,
                0,
            );
            self.transpose();
        }

        ui.add_space(10.0);

        #[cfg(not(target_arch = "wasm32"))]
        ui.horizontal(|ui| {
            if ui.button("play tab").clicked() {
                self.audio_context = AudioContext::new();
                harptabber::play_tab_in_key(
                    self.input_text.clone(),
                    self.input_tuning,
                    self.style,
                    &self.key,
                    &self.audio_context.sink,
                );
                self.audio_context.sink.play();
            }
            if ui.button("stop").clicked() {
                self.audio_context = AudioContext::new();
            }
        });

        ui.add_space(10.0);

        ui.collapsing("tab keyboard", |ui| {
            self.tabkeyboard(ui, tedit_output.response.id);
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
                TextEdit::singleline(&mut self.style_example)
                    .desired_width(350.0)
                    .interactive(false),
            );
        });
    }

    fn tuning_selector(&mut self, ui: &mut egui::Ui, is_input: bool) {
        let (mut tuning, label_name) = if is_input {
            (self.input_tuning, "input tuning")
        } else {
            (self.output_tuning, "output tuning")
        };

        egui::ComboBox::from_label(label_name)
            .selected_text(tuning)
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
                    "diminished",
                    "lucky 13 diminished",
                    "lucky 13 powerchromatic",
                    "melody maker",
                    "easy 3rd",
                    "4 hole richter",
                    "5 hole richter",
                ]
                .iter()
                {
                    if ui
                        .selectable_value(&mut tuning, tuning_text, *tuning_text)
                        .changed()
                    {
                        if is_input {
                            self.keyboard_layout = harptabber::get_tabkeyboard_layout(tuning);
                            self.input_tuning = tuning;

                            let (notes, duplicated) =
                                harptabber::tuning_to_notes_in_order(self.input_tuning);
                            self.notes_in_order = notes;
                            self.duplicated_notes = duplicated;
                        } else {
                            self.output_tuning = tuning;
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
            self.style_example = "-2 -2'' -3 4 -4 5 5o 6";
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::BBends, "b-bends")
            .clicked()
        {
            self.style_example = "-2 -2bb -3 4 -4 5 5o 6";
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::DrawDefault, "draw-default")
            .clicked()
        {
            self.style_example = "2 2'' 3 +4 4 +5 +5o +6";
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::Plus, "plus/minus")
            .clicked()
        {
            self.style_example = "-2 -2'' -3 +4 -4 +5 +5o +6";
            self.transpose();
        }
        if ui
            .selectable_value(&mut self.style, Style::Harpsurgery, "harpsurgery")
            .clicked()
        {
            self.style_example = "2D 2D'' 3D 4B 4D 5B 5B# 6B";
            self.transpose();
        }
    }

    fn insert_text_at_pos(&mut self, ui: &mut egui::Ui, text: &str, tedit_id: egui::Id) {
        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), tedit_id) {
            use egui::TextBuffer as _;
            if let Some(ccursor) = state.ccursor_range() {
                self.input_text.insert_text(text, ccursor.primary.index);
                let new_ccursor =
                    egui::text::CCursor::new(ccursor.primary.index + text.chars().count());
                state.set_ccursor_range(Some(egui::text::CCursorRange::one(new_ccursor)));
                state.store(ui.ctx(), tedit_id);
                ui.ctx().memory_mut(|mem| mem.request_focus(tedit_id)); // give focus back to the [`TextEdit`].
            }
        }
    }

    fn get_tabkeyboard_button_color(&self, ui: &egui::Ui, hole: &str) -> egui::ecolor::Color32 {
        let degree = harptabber::tab_to_scale_degree(
            hole,
            self.from_position,
            &self.notes_in_order,
            &self.duplicated_notes,
        );

        if let Some(scale) = self.selected_scale {
            let scale = self.scales.get(scale).unwrap();
            let is_scale_note = scale.contains(&degree);

            match (is_scale_note, ui.ctx().style().visuals.dark_mode) {
                (true, true) => egui::ecolor::Color32::DARK_GREEN,
                (true, false) => egui::ecolor::Color32::LIGHT_GREEN,
                (false, _) => ui.ctx().style().visuals.code_bg_color,
            }
        } else {
            ui.ctx().style().visuals.code_bg_color
        }
    }

    fn tabkeyboard(&mut self, ui: &mut egui::Ui, tedit_id: egui::Id) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                egui::ComboBox::from_id_source("no label")
                    .selected_text(format!("{:?}", self.display_as))
                    .width(80.0)
                    .show_ui(ui, |ui| {
                        for (option_enum, option_str) in [
                            (DisplayOption::Tabs, "Tabs"),
                            (DisplayOption::Notes, "Notes"),
                            (DisplayOption::Degrees, "Degrees"),
                        ] {
                            if ui
                                .selectable_value(&mut self.display_as, option_enum, option_str)
                                .changed()
                            {
                                self.generate_keyboard_text();
                            };
                        }
                    });

                let mut _space = 192.0;
                #[cfg(not(target_arch = "wasm32"))]
                {
                    _space -= 82.0;
                    ui.checkbox(&mut self.should_play_note, "play notes");
                }

                if self.display_as == DisplayOption::Notes || self.should_play_note {
                    egui::ComboBox::from_label("key")
                        .selected_text(&self.key)
                        .width(60.0)
                        .show_ui(ui, |ui| {
                            for note in [
                                "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
                            ]
                            .iter()
                            {
                                if ui
                                    .selectable_value(&mut self.key, note.to_string(), *note)
                                    .changed()
                                {
                                    self.generate_keyboard_text();
                                }
                            }
                        });
                    ui.add_space(_space);
                } else {
                    ui.add_space(_space + 95.0);
                }
                if ui
                    .add(Button::new(
                        RichText::new("return").text_style(TextStyle::Monospace),
                    ))
                    .clicked()
                {
                    self.insert_text_at_pos(ui, "\n", tedit_id);
                }

                if ui
                    .add(Button::new(
                        RichText::new("backspace").text_style(TextStyle::Monospace),
                    ))
                    .clicked()
                {
                    self.backspace(ui, tedit_id);
                    self.transpose();
                }
            });

            let mut should_generate_keyboard_text = false;
            egui::ComboBox::from_label("highlight scale")
                .selected_text(self.selected_scale.unwrap_or("none"))
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(&mut self.selected_scale, None, "none")
                        .changed()
                    {
                        should_generate_keyboard_text = true;
                    };
                    for scale in self.scales.keys() {
                        if ui
                            .selectable_value(&mut self.selected_scale, Some(scale), scale)
                            .changed()
                        {
                            should_generate_keyboard_text = true;
                        };
                    }
                });
            if should_generate_keyboard_text {
                self.generate_keyboard_text();
            }

            ui.label(&self.keyboard_text);

            let rows = self.keyboard_layout.clone();

            for (i, row) in rows.iter().enumerate() {
                ui.horizontal(|ui| {
                    for hole in row {
                        if hole.is_empty() {
                            ui.add_enabled(
                                false,
                                Button::new(
                                    RichText::new("     ").text_style(TextStyle::Monospace),
                                )
                                .fill(egui::ecolor::Color32::TRANSPARENT),
                            );
                        } else {
                            let display_note = match self.display_as {
                                DisplayOption::Notes => harptabber::tab_to_note(
                                    hole,
                                    &self.key,
                                    &self.notes_in_order,
                                    &self.duplicated_notes,
                                )
                                .to_owned(),
                                DisplayOption::Degrees => harptabber::tab_to_scale_degree(
                                    hole,
                                    self.from_position,
                                    &self.notes_in_order,
                                    &self.duplicated_notes,
                                )
                                .to_owned(),
                                DisplayOption::Tabs => {
                                    harptabber::change_tab_style_single(hole, self.style)
                                }
                            };

                            // determine color of button, depending on scale being highlighted
                            let color = self.get_tabkeyboard_button_color(ui, hole);

                            let hole = harptabber::change_tab_style_single(hole, self.style);

                            let text = format!("{:width$}", &display_note, width = 5);
                            if ui
                                .add(
                                    Button::new(
                                        RichText::new(text.as_str())
                                            .text_style(TextStyle::Monospace),
                                    )
                                    .fill(color),
                                )
                                .clicked()
                            {
                                self.insert_text_at_pos(ui, hole.as_str(), tedit_id);
                                self.insert_text_at_pos(ui, " ", tedit_id);
                                self.transpose();

                                #[cfg(not(target_arch = "wasm32"))]
                                if self.should_play_note {
                                    harptabber::play_tab_in_key(
                                        hole,
                                        self.input_tuning,
                                        self.style,
                                        &self.key,
                                        &self.audio_context.sink,
                                    );
                                }
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
                                Button::new(
                                    RichText::new(text.as_str()).text_style(TextStyle::Monospace),
                                ),
                            );
                        }
                    });
                }
            }
        });
    }

    fn backspace(&mut self, ui: &mut egui::Ui, tedit_id: egui::Id) {
        if let Some(mut state) = egui::TextEdit::load_state(ui.ctx(), tedit_id) {
            use egui::TextBuffer as _;
            if let Some(ccursor) = state.ccursor_range() {
                let mut deletion_start_point = ccursor.primary.index;
                deletion_start_point = deletion_start_point.saturating_sub(1);

                let chars: Vec<char> = self.input_text.chars().collect();
                while deletion_start_point > 0 && chars[deletion_start_point - 1] != ' ' {
                    deletion_start_point -= 1;
                }
                self.input_text
                    .delete_char_range(deletion_start_point..(ccursor.primary.index));

                let new_ccursor = egui::text::CCursor::new(deletion_start_point);
                state.set_ccursor_range(Some(egui::text::CCursorRange::one(new_ccursor)));
                state.store(ui.ctx(), tedit_id);
                ui.ctx().memory_mut(|mem| mem.request_focus(tedit_id)); // give focus back to the [`TextEdit`].
            } else {
                self.input_text.pop();
                let mut last = self.input_text.chars().last();
                while last.is_some() && last.unwrap() != ' ' {
                    self.input_text.pop();
                    last = self.input_text.chars().last();
                }
            }
        }
    }

    fn help_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("Help")
            .collapsible(false)
            .resizable(false)
            .open(&mut self.help_open)
            .show(ctx, |ui| {
                ui.label("- if a note is too high or low to be played, or would");
                ui.label("  require bending an overblow, it will appear as X");
                ui.add_space(10.0);
                ui.label("- don't forget spaces between notes");
                ui.add_space(10.0);
                ui.label("- set the tab style to the one you use, so the tab can");
                ui.label("  be interpreted correctly");
                ui.add_space(10.0);
                ui.label("- everything other than a valid note is ignored");
            });
    }

    fn about_window(&mut self, ctx: &egui::Context) {
        egui::Window::new("About")
            .collapsible(false)
            .resizable(false)
            .open(&mut self.about_open)
            .show(ctx, |ui| {
                ui.label("harptabber-gui");
                ui.add_space(10.0);
                ui.label("Copyright © 2021-2024");
                ui.label("Sebastian James Thuemmel");
                ui.add_space(10.0);
                ui.add(egui::Hyperlink::from_label_and_url(
                    "source code",
                    "https://github.com/Seebass22/harptabber",
                ));
                ui.add(egui::Hyperlink::from_label_and_url(
                    "web version & downloads",
                    "https://seebass22.itch.io/harmonica-tab-transposer",
                ));
            });
    }

    fn scale_menu(&mut self, ui: &mut egui::Ui) {
        ui.menu_button("Scales", |ui| {
            for scale in self.scales.keys() {
                if ui.button(scale).clicked() {
                    self.input_text = harptabber::scale_to_tab(
                        scale,
                        self.input_tuning,
                        self.from_position as i32,
                        self.style,
                    );
                    self.transpose();
                }
            }
        });
    }

    fn generate_keyboard_text(&mut self) {
        let mut text = String::new();
        if self.display_as == DisplayOption::Notes {
            text.push_str(&self.key);
            text.push(' ');
        }

        text.push_str(self.input_tuning);
        text.push_str(" harp");

        if let Some(scale) = &self.selected_scale {
            text.push_str(", ");
            if self.display_as == DisplayOption::Notes {
                let shift = ((self.from_position - 1) * 7).rem_euclid(12) as usize;

                let chromatic_notes = [
                    "C", "Db", "D", "Eb", "E", "F", "F#", "G", "Ab", "A", "Bb", "B",
                ];
                let mut index = chromatic_notes
                    .iter()
                    .position(|note| *note == self.key)
                    .unwrap();
                index = (index + shift).rem_euclid(12);

                text.push_str(chromatic_notes[index]);
                text.push(' ');
            } else {
                let pos_string = harptabber::to_ordinal(self.from_position);
                text.push_str(&format!("{} position ", pos_string));
            }

            text.push_str(scale);
            text.push_str(" scale highlighted");
        }

        self.keyboard_text = text;
    }
}
