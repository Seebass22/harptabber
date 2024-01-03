use std::collections::BTreeSet;

use eframe::{egui, epaint::Color32};

#[derive(Default)]
pub struct MemoizedHighlighter {
    style: egui::Style,
    text: String,
    errors: Vec<String>,
    output: egui::text::LayoutJob,
}

impl MemoizedHighlighter {
    pub fn highlight(
        &mut self,
        egui_style: &egui::Style,
        text: &str,
        invalid_notes: &[String],
    ) -> egui::text::LayoutJob {
        if (&self.style, self.text.as_str(), self.errors.as_slice()) != (egui_style, text, invalid_notes) {
            self.style = egui_style.clone();
            self.text = text.to_owned();
            self.errors = invalid_notes.to_vec();
            self.output = highlight_tab(egui_style, text, invalid_notes);
        }
        self.output.clone()
    }
}

fn highlight_tab(
    egui_style: &egui::Style,
    mut text: &str,
    invalid_notes: &[String],
) -> egui::text::LayoutJob {
    dbg!(invalid_notes);
    let mut job = egui::text::LayoutJob::default();

    let mut prev_pos = 0;
    // let matches = [(3usize, 8usize), (10, 18)];

    let mut indices: BTreeSet<(usize, usize)> = invalid_notes.iter().map(|invalid_note| {
        text
            .match_indices(invalid_note)
            .map(|(i, string)| (i, i + string.len()))
    }).flatten().collect();
    dbg!(&indices);

    
    let indices_copy = indices.clone();
    indices.retain(|(start, stop)| !indices_copy.iter().any(|(other_start, other_stop)| {
        (start >= other_start && stop <= other_stop) && !(start == other_start && stop == other_stop)
    }));

    if text.len() > 19 {
        for &(start, stop) in indices.iter() {
            job.append(
                &text[prev_pos..start],
                0.0,
                egui::text::TextFormat {
                    color: Color32::WHITE,
                    ..Default::default()
                },
            );

            job.append(
                &text[start..stop],
                0.0,
                egui::text::TextFormat {
                    color: Color32::RED,
                    ..Default::default()
                },
            );
            prev_pos = stop;
        }
        job.append(
            &text[prev_pos..],
            0.0,
            egui::text::TextFormat {
                color: Color32::WHITE,
                ..Default::default()
            },
        );
    } else {
        job.append(
            &text[..],
            0.0,
            egui::text::TextFormat {
                color: Color32::WHITE,
                ..Default::default()
            },
        );
    }

    job
}
