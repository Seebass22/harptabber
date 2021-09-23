use std::fs;

fn transpose<'a>(notes: &'a [&str], note: &str, semitones: i32) -> &'a str {
    let mut pos = notes.iter().position(|&x| x == note).unwrap() as i32;
    pos += semitones;
    if let Some(new_note) = notes.get(pos as usize) {
        new_note
    } else {
        eprintln!("could not transpose: exceeded harmonica bounds");
        std::process::exit(-2);
    }
}

fn transpose_position<'a>(
    notes: &'a [&str],
    note: &str,
    from_position: i32,
    to_position: i32,
    octave_shift: i32,
) -> &'a str {
    let diff = to_position - from_position;
    let semitones = ((diff * 7) % 12) + 12 * octave_shift;
    transpose(notes, note, semitones)
}

fn readfile(filename: &str) -> Vec<Vec<String>> {
    let contents;
    match fs::read_to_string(filename) {
        Ok(s) => contents = s,
        Err(_) => {
            eprintln!("could not read file");
            std::process::exit(-1);
        }
    }
    let mut lines: Vec<Vec<String>> = Vec::new();
    for line in contents.lines() {
        let line: Vec<String> = line.split_whitespace().map(|s| s.to_string()).collect();
        lines.push(line);
    }
    lines
}

pub fn run(
    filename: &str,
    semitones: i32,
    from_position: i32,
    to_position: Option<i32>,
    octave_shift: i32,
) {
    let notes = [
        "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
        "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
        "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10",
    ];

    let contents = readfile(filename);

    for line in contents {
        for note in line {
            let new_note;
            if let Some(to_position) = to_position {
                new_note = transpose_position(
                    &notes,
                    note.as_str(),
                    from_position,
                    to_position,
                    octave_shift,
                );
            } else {
                new_note = transpose(&notes, note.as_str(), semitones);
            }
            print!("{} ", new_note);
        }
        println!();
    }
}
