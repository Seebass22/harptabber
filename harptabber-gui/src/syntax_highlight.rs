use eframe::{egui, epaint::Color32};

#[derive(Default)]
pub struct MemoizedHighlighter {
    style: egui::Style,
    text: String,
    output: egui::text::LayoutJob,
}

impl MemoizedHighlighter {
    pub fn highlight(&mut self, egui_style: &egui::Style, text: &str) -> egui::text::LayoutJob {
        if (&self.style, self.text.as_str()) != (egui_style, text) {
            self.style = egui_style.clone();
            self.text = text.to_owned();
            self.output = highlight_tab(egui_style, text);
        }
        self.output.clone()
    }
}

pub fn highlight_tab(egui_style: &egui::Style, mut text: &str) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();

    let color = Color32::LIGHT_RED;
    job.append(
        &text[..],
        0.0,
        egui::text::TextFormat {
            color,
            ..Default::default()
        },
    );
    job
}
