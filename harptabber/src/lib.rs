use regex::Regex;
use std::collections::HashMap;
use std::fs;
#[macro_use]
extern crate lazy_static;

pub mod adsr;
pub mod audioplayback;

#[derive(PartialEq, Copy, Clone)]
pub enum Style {
    Default,
    Harpsurgery,
    BBends,
    Plus,
    DrawDefault,
}

fn transpose<'a>(
    input_harp_notes: &'a [String],
    output_harp_notes: &'a [String],
    note: &str,
    semitones: i32,
) -> Result<&'a str, String> {
    let mut pos;
    match input_harp_notes.iter().position(|x| x == note) {
        Some(p) => pos = p as i32,
        None => {
            let error = format!("\"{}\" is not a valid note", note);
            return Err(error);
        }
    }

    pos += semitones;
    if let Some(new_note) = output_harp_notes.get(pos as usize) {
        Ok(new_note)
    } else {
        Ok("X")
    }
}

pub fn semitones_to_position(starting_pos: u32, semitones: i32) -> u32 {
    let position_diffs = [0, 7, 2, 9, 4, 11, 6, 1, 8, 3, 10, 5];
    let index = semitones.rem_euclid(12);
    let res = position_diffs[index as usize];
    (res + starting_pos - 1).rem_euclid(12) + 1
}

pub fn positions_to_semitones(from_position: i32, to_position: i32, octave_shift: i32) -> i32 {
    let diff = to_position - from_position;
    (diff * 7).rem_euclid(12) + 12 * octave_shift
}

pub struct RunOptions<'a> {
    pub filename: &'a str,
    pub semitones: i32,
    pub from_position: i32,
    pub to_position: Option<i32>,
    pub octave_shift: i32,
    pub no_error: bool,
    pub style: Style,
    pub input_tuning: &'a str,
    pub output_tuning: &'a str,
}

pub fn run(options: RunOptions) {
    let RunOptions {
        filename,
        mut semitones,
        from_position,
        to_position,
        octave_shift,
        no_error,
        style,
        input_tuning,
        output_tuning,
    } = options;

    let tab = match fs::read_to_string(filename) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("could not read file");
            std::process::exit(-1);
        }
    };

    if let Some(to_position) = to_position {
        semitones = positions_to_semitones(from_position, to_position, octave_shift);
    } else if octave_shift != 0 {
        semitones = positions_to_semitones(from_position, 1, octave_shift);
    }

    let tabs = transpose_tabs(tab, semitones, no_error, style, input_tuning, output_tuning);
    print!("{}", tabs);
}

fn convert_to_plus_style(note: &str) -> String {
    match note.chars().next().unwrap() {
        '-' => note.to_string(),
        _ => {
            let mut res = String::from("+");
            res.push_str(note);
            res
        }
    }
}

fn convert_to_draw_style(note: &str) -> String {
    match note.chars().next().unwrap() {
        '-' => note[1..].to_string(),
        _ => {
            let mut res = String::from("+");
            res.push_str(note);
            res
        }
    }
}

fn convert_to_harpsurgery_style(note: &str) -> String {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(?P<dir>-?)(?P<note>\d{1,2})(?P<rest>.*)").unwrap();
    }

    let caps = RE.captures(note).unwrap();

    let dir = if &caps["dir"] == "-" { "D" } else { "B" };
    let rest = if &caps["rest"] == "o" {
        "#"
    } else {
        &caps["rest"]
    };

    format!("{}{}{}", &caps["note"], dir, rest)
}

pub fn change_tab_style_single(note: &str, style: Style) -> String {
    match style {
        Style::BBends => note.replace("'", "b"),
        Style::Harpsurgery => convert_to_harpsurgery_style(note),
        Style::Plus => convert_to_plus_style(note),
        Style::DrawDefault => convert_to_draw_style(note),
        _ => note.to_string(),
    }
}

fn change_tab_style<T: AsRef<str>>(notes: &[T], style: Style) -> Vec<String> {
    notes
        .iter()
        .map(|s| change_tab_style_single(s.as_ref(), style))
        .collect()
}

fn fix_enharmonics<'a>(note: &'a str, duplicated_notes: &'a [String]) -> &'a str {
    let mut i = 0;
    while i < duplicated_notes.len() {
        if note == duplicated_notes.get(i).unwrap() {
            return duplicated_notes.get(i + 1).unwrap();
        }
        i += 2;
    }
    note
}

pub fn tuning_to_notes(tuning: &str) -> &'static str {
    let mut tunings = HashMap::<&str, &str>::new();
    tunings.insert("richter", "C E G C E G C E G C\nD G B D F A B D F A\n");
    tunings.insert("country", "C E G C E G C E G C\nD G B D F# A B D F A\n");
    tunings.insert("wilde tuning", "C E G C E E G C E A\nD G B D F G B D G C\n");
    tunings.insert(
        "melody maker",
        "C E A C E G C E G C\nD G B D F# A B D F# A\n",
    );
    tunings.insert(
        "natural minor",
        "C Eb G C Eb G C Eb G C\nD G Bb D F A Bb D F A\n",
    );
    tunings.insert(
        "harmonic minor",
        "C Eb G C Eb G C Eb G C\nD G B D F Ab B D F Ab\n",
    );
    tunings.insert(
        "paddy richter",
        "C E A C E G C E G C\nD G B D F A B D F A\n",
    );
    tunings.insert("pentaharp", "A D E A D E A D E A\nC Eb G C Eb G C Eb G C");
    tunings.insert("powerdraw", "C E G C E G A C E A\nD G B D F A B D G C");
    tunings.insert("powerbender", "C E G C D F A C E A\nD G B D E G B D G C");
    tunings.insert("easy 3rd", "C E G C E G C E G C\nD F A D F A B D F A");
    tunings.insert("4 hole richter", "C E G C\nD F A B");

    match tunings.get(tuning) {
        Some(tuning) => tuning,
        None => {
            eprintln!("tuning not found: {}", tuning);
            tunings.get("richter").unwrap()
        }
    }
}

fn tuning_to_notes_in_order(tuning: &str) -> (Vec<String>, Vec<String>) {
    let notes = tuning_to_notes(tuning);
    harptool::str_to_notes_in_order(notes)
}

pub fn transpose_tabs(
    tab: String,
    semitones: i32,
    no_error: bool,
    style: Style,
    input_tuning: &str,
    output_tuning: &str,
) -> String {
    let (input_notes, duplicated_notes) = tuning_to_notes_in_order(input_tuning);
    let input_notes = change_tab_style(&input_notes, style);
    let duplicated_notes = change_tab_style(&duplicated_notes, style);

    let (output_notes, _) = tuning_to_notes_in_order(output_tuning);
    let output_notes = change_tab_style(&output_notes, style);

    let mut result = String::from("");

    for line in tab.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        for note in line {
            let note = fix_enharmonics(note, &duplicated_notes);
            let new_note = transpose(&input_notes, &output_notes, note, semitones);

            match new_note {
                Ok(new_note) => {
                    result.push_str(new_note);
                    result.push(' ');
                }
                Err(s) => {
                    if !no_error {
                        eprintln!("{}", s);
                        std::process::exit(-1);
                    }
                }
            }
        }
        result.push('\n');
    }
    result
}

pub fn get_tabkeyboard_layout(input_tuning: &str) -> Vec<Vec<String>> {
    let notes = tuning_to_notes(input_tuning);
    let tuning = harptool::Tuning::from(notes);
    let harplen = tuning.blow.len();

    let mut blow: Vec<String> = Vec::new();
    let mut draw: Vec<String> = Vec::new();
    let mut blow_bends_1: Vec<String> = Vec::new();
    let mut blow_bends_2: Vec<String> = Vec::new();
    let mut draw_bends_1: Vec<String> = Vec::new();
    let mut draw_bends_2: Vec<String> = Vec::new();
    let mut draw_bends_3: Vec<String> = Vec::new();

    for i in 0..harplen {
        blow.push((i + 1).to_string());
        blow_bends_1.push("".to_string());
        blow_bends_2.push("".to_string());
        draw_bends_1.push("".to_string());
        draw_bends_2.push("".to_string());
        draw_bends_3.push("".to_string());
    }
    for i in 0..harplen {
        let i = -((i + 1) as i32);
        draw.push((i).to_string());
    }
    for i in 0..harplen {
        let mut note = (i + 1).to_string();
        if tuning.blow_bends_half.get(i).unwrap().is_some() {
            note.push('\'');
            *blow_bends_1.get_mut(i).unwrap() = note;
        } else if tuning.overblows.get(i).unwrap().is_some() {
            note.push('o');
            *blow_bends_1.get_mut(i).unwrap() = note;
        }
    }

    for i in 0..harplen {
        let mut note = (i + 1).to_string();
        if tuning.blow_bends_full.get(i).unwrap().is_some() {
            note.push_str("''");
            *blow_bends_2.get_mut(i).unwrap() = note;
        }
    }

    for i in 0..harplen {
        let index = i as i32;
        let mut note = (-(index + 1)).to_string();
        if tuning.bends_half.get(i).unwrap().is_some() {
            note.push('\'');
            *draw_bends_1.get_mut(i).unwrap() = note;
        } else if tuning.overdraws.get(i).unwrap().is_some() {
            note.push('o');
            *draw_bends_1.get_mut(i).unwrap() = note;
        }
    }

    for i in 0..harplen {
        let index = i as i32;
        let mut note = (-(index + 1)).to_string();
        if tuning.bends_full.get(i).unwrap().is_some() {
            note.push_str("''");
            *draw_bends_2.get_mut(i).unwrap() = note;
        }
    }

    for i in 0..harplen {
        let index = i as i32;
        let mut note = (-(index + 1)).to_string();
        if tuning.bends_one_and_half.get(i).unwrap().is_some() {
            note.push_str("'''");
            *draw_bends_3.get_mut(i).unwrap() = note;
        }
    }

    vec![
        blow_bends_2,
        blow_bends_1,
        blow,
        draw,
        draw_bends_1,
        draw_bends_2,
        draw_bends_3,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_tabkeyboard_layout() {
        let res = get_tabkeyboard_layout("richter");
        let expected = vec![
            vec!["", "", "", "", "", "", "", "", "", "10''"],
            vec!["1o", "", "", "4o", "5o", "6o", "", "8'", "9'", "10'"],
            vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            vec!["-1", "-2", "-3", "-4", "-5", "-6", "-7", "-8", "-9", "-10"],
            vec![
                "-1'", "-2'", "-3'", "-4'", "", "-6'", "-7o", "", "-9o", "-10o",
            ],
            vec!["", "-2''", "-3''", "", "", "", "", "", "", ""],
            vec!["", "", "-3'''", "", "", "", "", "", "", ""],
        ];
        assert_eq!(res, expected);

        let res = get_tabkeyboard_layout("asdf");
        assert_eq!(res, expected);

        let res = get_tabkeyboard_layout("wilde tuning");
        let expected = vec![
            vec!["", "", "", "", "", "", "", "", "", ""],
            vec!["1o", "", "", "4o", "", "", "", "8o", "9o", "10o"],
            vec!["1", "2", "3", "4", "5", "6", "7", "8", "9", "10"],
            vec!["-1", "-2", "-3", "-4", "-5", "-6", "-7", "-8", "-9", "-10"],
            vec![
                "-1'", "-2'", "-3'", "-4'", "", "-6'", "-7'", "-8'", "-9'", "-10'",
            ],
            vec![
                "", "-2''", "-3''", "", "", "-6''", "-7''", "", "-9''", "-10''",
            ],
            vec!["", "", "-3'''", "", "", "", "-7'''", "", "", ""],
        ];
        assert_eq!(res, expected);
    }

    #[test]
    fn test_transpose_tabs() {
        let tab = String::from("-2 -3'' -3 4 -4 5 5o 6");

        // down 5th (G -> C)
        let res = transpose_tabs(tab.clone(), -7, true, Style::Default, "richter", "richter");
        assert_eq!(res.as_str(), "1 -1 2 -2'' -2 -3'' -3 4 \n");

        // down 5th, up octave (G -> C)
        let res = transpose_tabs(
            tab.clone(),
            -7 + 12,
            true,
            Style::Default,
            "richter",
            "richter",
        );
        assert_eq!(res.as_str(), "4 -4 5 -5 6 -6 -7 7 \n");

        // up 5th, down octave (G -> D)
        let res = transpose_tabs(tab, 7 - 12, true, Style::Default, "richter", "richter");
        assert_eq!(res.as_str(), "-1 2 -2' -2 -3'' -3 -4' -4 \n");

        // test enharmonics
        let res = transpose_tabs(
            "3B".to_string(),
            12,
            true,
            Style::Harpsurgery,
            "richter",
            "richter",
        );
        assert_eq!(res.as_str(), "6B \n");
    }

    #[test]
    fn test_transpose_tabs_different_tunings() {
        let res = transpose_tabs(
            "-2 -3' 4 -4' -4 -5 6".to_string(),
            0,
            true,
            Style::Default,
            "richter",
            "wilde tuning",
        );
        assert_eq!(res.as_str(), "-2 -3' 4 -4' -4 -5 -6 \n");

        let res = transpose_tabs(
            "-2 -3' 4 -4' -4 -5 6".to_string(),
            12,
            true,
            Style::Default,
            "richter",
            "wilde tuning",
        );
        assert_eq!(res.as_str(), "-6 -7' 8 -8' -8 -9'' -9 \n");

        let res = transpose_tabs(
            "-3'' -3 4 -4 5 -5 6 -6".to_string(),
            -2,
            true,
            Style::Default,
            "richter",
            "natural minor",
        );
        assert_eq!(res.as_str(), "-2 -3' -3 4 -4 5 -5 6 \n");
    }

    #[test]
    fn test_easy_3rd_tuning() {
        let input = "-3'' -3 4 -4 5 -5 6 -6";
        let res = transpose_tabs(
            input.to_string(),
            5,
            true,
            Style::Default,
            "richter",
            "easy 3rd",
        );
        assert_eq!(res.as_str(), "-4 5 -5 6 -6 6o 7 -8 \n");

        let input = "-3'' -3 4 -4 5 -5 6 -6";
        let res = transpose_tabs(
            input.to_string(),
            -7,
            true,
            Style::Default,
            "richter",
            "easy 3rd",
        );
        assert_eq!(res.as_str(), "-1 2 -2 3 -3 3o 4 -4 \n");

        let input = "1 -1 2 -2'' -2 -3'' -3 4";
        let res = transpose_tabs(
            input.to_string(),
            0,
            true,
            Style::Default,
            "richter",
            "easy 3rd",
        );
        assert_eq!(res.as_str(), "1 -1 2 -2 3 -3 X 4 \n");
    }

    #[test]
    fn test_transpose() {
        let notes = [
            "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4",
            "-4'", "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8",
            "8'", "8", "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10", "-10o",
        ];
        let notes: Vec<String> = notes.iter().map(|s| s.to_string()).collect();

        let note = "1";
        let res = transpose(&notes, &notes, note, 1);
        assert_eq!(res, Ok("-1'"));

        let note = "1";
        let res = transpose(&notes, &notes, note, -1);
        assert_eq!(res, Ok("X"));

        let note = "10";
        let res = transpose(&notes, &notes, note, 2);
        assert_eq!(res, Ok("X"));

        let note = "asdf";
        let res = transpose(&notes, &notes, note, -1);
        assert!(res.is_err());
    }

    #[test]
    fn test_semitones_to_position() {
        let res = semitones_to_position(1, 7);
        assert_eq!(res, 2);

        let res = semitones_to_position(1, -7);
        assert_eq!(res, 12);

        let res = semitones_to_position(1, 2);
        assert_eq!(res, 3);

        let res = semitones_to_position(2, 7);
        assert_eq!(res, 3);

        let res = semitones_to_position(3, -3);
        assert_eq!(res, 6);

        let res = semitones_to_position(3, 9);
        assert_eq!(res, 6);

        let res = semitones_to_position(1, 12);
        assert_eq!(res, 1);

        let res = semitones_to_position(1, 0);
        assert_eq!(res, 1);

        let res = semitones_to_position(2, -7);
        assert_eq!(res, 1);
    }

    #[test]
    fn test_convert_to_harpsurgery_style() {
        let res = convert_to_harpsurgery_style("8'");
        assert_eq!(res.as_str(), "8B'");

        let res = convert_to_harpsurgery_style("-2''");
        assert_eq!(res.as_str(), "2D''");

        let res = convert_to_harpsurgery_style("4o");
        assert_eq!(res.as_str(), "4B#");

        let res = convert_to_harpsurgery_style("-7o");
        assert_eq!(res.as_str(), "7D#");

        let res = convert_to_harpsurgery_style("10''");
        assert_eq!(res.as_str(), "10B''");
    }

    #[test]
    fn test_convert_to_draw_style() {
        let res = convert_to_draw_style("4");
        assert_eq!(res.as_str(), "+4");

        let res = convert_to_draw_style("-4'");
        assert_eq!(res.as_str(), "4'");

        let res = convert_to_draw_style("5o");
        assert_eq!(res.as_str(), "+5o");
    }

    #[test]
    fn test_change_tab_style() {
        let input = ["-2", "-3'", "4", "-4'", "-4", "-5", "6"];

        let res = change_tab_style(&input, Style::BBends);
        let expected = ["-2", "-3b", "4", "-4b", "-4", "-5", "6"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(res, expected);

        let res = change_tab_style(&input, Style::Harpsurgery);
        let expected = ["2D", "3D'", "4B", "4D'", "4D", "5D", "6B"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(res, expected);

        let res = change_tab_style(&input, Style::DrawDefault);
        let expected = ["2", "3'", "+4", "4'", "4", "5", "+6"]
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>();
        assert_eq!(res, expected);
    }
}
