use eframe::egui;
use regex::Regex;
use std::collections::BTreeSet;

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
        if (&self.style, self.text.as_str(), self.errors.as_slice())
            != (egui_style, text, invalid_notes)
        {
            self.style = egui_style.clone();
            text.clone_into(&mut self.text);
            self.errors = invalid_notes.to_vec();
            self.output = highlight_tab(egui_style, text, invalid_notes);
        }
        self.output.clone()
    }
}

fn highlight_tab(
    egui_style: &egui::Style,
    text: &str,
    invalid_notes: &[String],
) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();

    // workaround: double quotes may have been replaced by single quotes in input
    let mut new_invalid_notes = Vec::new();
    for string in invalid_notes.iter() {
        new_invalid_notes.push(string.clone());
        if string.contains("''") {
            new_invalid_notes.push(string.replace("''", "\""));
        }
    }

    let mut indices: BTreeSet<(usize, usize)> = new_invalid_notes
        .iter()
        .flat_map(|invalid_note| {
            let escaped = regex::escape(invalid_note);
            // prevent highlighting parts of valid notes (e.g. ' in -4' or 2' in -2')
            let regex_string = r"(^|[^0-9'-])(?<target>".to_owned() + &escaped + ")";
            let re = Regex::new(&regex_string).unwrap();
            re.captures_iter(text)
                .map(|captures| captures.name("target").unwrap())
                .map(|m| (m.start(), m.end()))
                .collect::<Vec<(usize, usize)>>()
        })
        .collect();

    let indices_copy = indices.clone();
    indices.retain(|(start, stop)| {
        !indices_copy.iter().any(|(other_start, other_stop)| {
            (start >= other_start && stop <= other_stop)
                && !(start == other_start && stop == other_stop)
        })
    });

    let default_text_format = egui::text::TextFormat {
        color: egui_style.visuals.text_color(),
        ..Default::default()
    };
    let error_text_format = egui::text::TextFormat {
        color: egui_style.visuals.error_fg_color,
        ..Default::default()
    };

    let mut prev_pos = 0;
    if !indices.is_empty() {
        for &(start, stop) in indices.iter() {
            // fix invalid range indices sometimes occurring
            if prev_pos > start {
                job.append(&text[prev_pos..], 0.0, default_text_format.clone());
                break;
            }
            job.append(&text[prev_pos..start], 0.0, default_text_format.clone());
            job.append(&text[start..stop], 0.0, error_text_format.clone());
            prev_pos = stop;
        }
        job.append(&text[prev_pos..], 0.0, default_text_format);
    } else {
        job.append(text, 0.0, default_text_format);
    }

    job
}
