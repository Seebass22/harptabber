use clap::{App, Arg};

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
        .get_matches();

    let semitones = matches.value_of("semitones").unwrap_or("0");
    let semitones = semitones.parse::<i32>().unwrap();

    let notes = [
        "1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3", "4", "-4'",
        "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o", "-7", "7", "-7o", "-8", "8'", "8",
        "-9", "9'", "9", "-9o", "-10", "10''", "10'", "10",
    ];

    let tabs = ["-2", "-3", "4", "-4'", "-4", "-4"];

    for note in tabs {
        let new_note = transpose(&notes, note, semitones);
        print!("{} ", new_note);
    }
    println!();

    transpose(&notes, "1", 11);
}
