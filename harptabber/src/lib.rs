use regex::Regex;
use std::fs;
#[macro_use]
extern crate lazy_static;

#[derive(PartialEq)]
pub enum Style {
    Default,
    Harpsurgery,
    B,
}

fn transpose<'a>(notes: &'a [String], note: &str, semitones: i32) -> Result<&'a str, String> {
    let mut pos;
    match notes.iter().position(|x| x == note) {
        Some(p) => pos = p as i32,
        None => {
            let error = format!("\"{}\" is not a valid note", note);
            return Err(error);
        }
    }

    pos += semitones;
    if let Some(new_note) = notes.get(pos as usize) {
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

pub fn run(
    filename: &str,
    mut semitones: i32,
    from_position: i32,
    to_position: Option<i32>,
    octave_shift: i32,
    no_error: bool,
    style: Style,
) {
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

    let tabs = transpose_tabs(tab, semitones, no_error, style);
    print!("{}", tabs);
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

fn change_tab_style(notes: &[&str], style: Style) -> Vec<String> {
    let notes: Vec<String> = match style {
        Style::B => notes.iter().map(|s| s.replace("'", "b")).collect(),
        Style::Harpsurgery => notes
            .iter()
            .map(|s| convert_to_harpsurgery_style(s))
            .collect(),
        _ => notes.iter().map(|s| s.to_string()).collect(),
    };

    notes
}

pub fn transpose_tabs(tab: String, semitones: i32, no_error: bool, style: Style) -> String {
    let notes = [
        "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
        "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
        "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10", "-10o",
    ];
    let notes = change_tab_style(&notes, style);

    let mut result = String::from("");

    for line in tab.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        for note in line {
            let new_note = transpose(&notes, note, semitones);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose_tabs() {
        let tab = String::from("-2 -3'' -3 4 -4 5 5o 6");

        // down 5th (G -> C)
        let res = transpose_tabs(tab.clone(), -7, true, Style::Default);
        assert_eq!(res.as_str(), "1 -1 2 -2'' -2 -3'' -3 4 \n");

        // down 5th, up octave (G -> C)
        let res = transpose_tabs(tab.clone(), -7 + 12, true, Style::Default);
        assert_eq!(res.as_str(), "4 -4 5 -5 6 -6 -7 7 \n");

        // up 5th, down octave (G -> D)
        let res = transpose_tabs(tab, 7 - 12, true, Style::Default);
        assert_eq!(res.as_str(), "-1 2 -2' -2 -3'' -3 -4' -4 \n");
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
        let res = transpose(&notes, note, 1);
        assert_eq!(res, Ok("-1'"));

        let note = "1";
        let res = transpose(&notes, note, -1);
        assert_eq!(res, Ok("X"));

        let note = "10";
        let res = transpose(&notes, note, 2);
        assert_eq!(res, Ok("X"));

        let note = "asdf";
        let res = transpose(&notes, note, -1);
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
}
