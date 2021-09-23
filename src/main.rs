use clap::{App, Arg};
use std::fs;

fn transpose<'a>(notes: &'a [&str], note: &str, semitones: i32) -> &'a str {
    let mut pos = notes.iter().position(|&x| x == note).unwrap() as i32;
    pos += semitones;
    let new_note = notes
        .get(pos as usize)
        .expect("cannot transpose: exceeded bounds");
    new_note
}

fn main() {
    let matches = App::new("harptool")
        .about("print harmonica note layouts")
        .arg(
            Arg::with_name("semitones")
                .short("s")
                .long("semitones")
                .value_name("SEMITONES")
                .help("number of semitones to transpose"),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("file containing tabs")
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap_or("tabs.txt");
    let contents;
    match fs::read_to_string(filename) {
        Ok(s) => contents = s,
        Err(_) => {
            eprintln!("could not read file");
            panic!();
        }
    }
    let contents: Vec<&str> = contents.split_whitespace()
        .collect();

    let semitones = matches.value_of("semitones").unwrap_or("0");
    let semitones = semitones.parse::<i32>().unwrap();

    let notes = [
        "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
        "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
        "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10",
    ];

    for note in contents {
        let new_note = transpose(&notes, note, semitones);
        print!("{} ", new_note);
    }
    println!();
}
