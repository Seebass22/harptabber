use clap::{App, Arg};
use harptabber::run;

fn is_int(val: String) -> Result<(), String> {
    if val.parse::<i32>().is_ok() {
        Ok(())
    } else {
        Err(String::from("arg must be integer"))
    }
}

fn main() {
    let matches = App::new("harptabber")
        .about("transpose harmonica tabs")
        .arg(
            Arg::with_name("semitones")
                .short("s")
                .long("semitones")
                .value_name("SEMITONES")
                .allow_hyphen_values(true)
                .help("number of semitones to transpose")
                .validator(is_int),
        )
        .arg(
            Arg::with_name("to-position")
                .short("t")
                .long("to")
                .value_name("POSITION")
                .help("position to transpose to")
                .validator(is_int),
        )
        .arg(
            Arg::with_name("from-position")
                .short("f")
                .long("from")
                .value_name("POSITION")
                .default_value("1")
                .help("position to transpose from")
                .validator(is_int),
        )
        .arg(
            Arg::with_name("octave-shift")
                .short("o")
                .long("octave")
                .value_name("OCTAVES")
                .allow_hyphen_values(true)
                .default_value("0")
                .help("octaves to shift resulting tab")
                .validator(is_int),
        )
        .arg(
            Arg::with_name("no-error")
                .short("e")
                .long("no-error")
                .help("ignore invalid notes"),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("file containing tabs")
                .required(true),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let semitones = matches.value_of("semitones").unwrap_or("0");
    let semitones = semitones.parse::<i32>().unwrap();
    let no_error = matches.is_present("no-error");
    let from_position = matches
        .value_of("from-position")
        .unwrap()
        .parse::<i32>()
        .unwrap();
    let octave_shift = matches
        .value_of("octave-shift")
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let mut to_position = None;
    if matches.is_present("to-position") {
        to_position = matches.value_of("to-position").unwrap().parse::<i32>().ok();
    }

    run(
        filename,
        semitones,
        from_position,
        to_position,
        octave_shift,
        no_error,
    );
}
