use eframe::egui;
use harptabber::Style;

pub fn tabkeyboard(
    ui: &mut egui::Ui,
    input_text: &mut String,
    output_text: &mut String,
    semitone_shift: &i32,
    style: &Style,
) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add_space(373.0);
            if ui
                .add(egui::Button::new("return").text_style(egui::TextStyle::Monospace))
                .clicked()
            {
                input_text.push_str("\n");
            }

            if ui
                .add(egui::Button::new("backspace").text_style(egui::TextStyle::Monospace))
                .clicked()
            {
                input_text.pop();
                let mut last = input_text.chars().last();
                while last.is_some() && last.unwrap() != ' ' {
                    input_text.pop();
                    last = input_text.chars().last();
                }
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
                        let hole = harptabber::change_tab_style_single(hole, *style);

                        let text = format!("{:width$}", hole, width = 5);
                        if ui
                            .add(
                                egui::Button::new(text.as_str())
                                    .text_style(egui::TextStyle::Monospace),
                            )
                            .clicked()
                        {
                            input_text.push_str(hole.as_str());
                            input_text.push_str(" ");
                            *output_text = harptabber::transpose_tabs(
                                input_text.clone(),
                                *semitone_shift,
                                true,
                                *style,
                            );
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
