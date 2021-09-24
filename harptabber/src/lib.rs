use std::fs;

fn transpose<'a>(notes: &'a [&str], note: &str, semitones: i32) -> Result<&'a str, String> {
    let mut pos;
    match notes.iter().position(|&x| x == note) {
        Some(p) => pos = p as i32,
        None => {
            let error = format!("\"{}\" is not a valid note", note);
            return Err(String::from(error));
        }
    }

    pos += semitones;
    if let Some(new_note) = notes.get(pos as usize) {
        Ok(new_note)
    } else {
        Ok("X")
    }
}

fn positions_to_semitones(from_position: i32, to_position: i32, octave_shift: i32) -> i32 {
    let diff = to_position - from_position;
    let semitones = ((diff * 7) % 12) + 12 * octave_shift;
    semitones
}

pub fn run(
    filename: &str,
    mut semitones: i32,
    from_position: i32,
    to_position: Option<i32>,
    octave_shift: i32,
    no_error: bool,
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

    let tabs = transpose_tabs(tab, semitones, no_error);
    print!("{}", tabs);
}

fn transpose_tabs(tab: String, semitones: i32, no_error: bool) -> String {
    let notes = [
        "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
        "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
        "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10", "-10o",
    ];

    let mut result = String::from("");

    for line in tab.lines() {
        let line: Vec<&str> = line.split_whitespace().collect();

        for note in line {
            let new_note = transpose(&notes, note, semitones);

            match new_note {
                Ok(new_note) => {
                    result.push_str(new_note);
                    result.push_str(" ");
                }
                Err(s) => {
                    if !no_error {
                        eprintln!("{}", s);
                        std::process::exit(-1);
                    }
                }
            }
        }
        result.push_str("\n");
    }
    result
}
