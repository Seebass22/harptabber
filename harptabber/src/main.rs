use clap::{App, Arg};
use harptabber::{run, RunOptions, Style};

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
            Arg::with_name("keep-errors")
                .short("e")
                .long("keep-errors")
                .help("include invalid notes in output tab"),
        )
        .arg(
            Arg::with_name("style")
                .long("style")
                .value_name("STYLE")
                .help("set tab style (harpsurgery, b-bends, plus, draw, default)"),
        )
        .arg(
            Arg::with_name("input-tuning")
                .long("input-tuning")
                .value_name("TUNING")
                .default_value("richter")
                .help("set tuning of input harp"),
        )
        .arg(
            Arg::with_name("output-tuning")
                .long("output-tuning")
                .value_name("TUNING")
                .default_value("richter")
                .help("set tuning of output harp"),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("file containing tabs")
                .required(true),
        )
        .arg(
            Arg::with_name("playable-positions")
                .short("p")
                .long("playable-positions")
                .help("transpose to all playable positions (without overblows)"),
        )
        .arg(
            Arg::with_name("no-bends")
                .short("n")
                .long("no-bends")
                .help("disallow bends (with --playable-positions)"),
        )
        .arg(
            Arg::with_name("play")
                .short("a")
                .long("play")
                .help("play tab as audio"),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap();
    let semitones = matches.value_of("semitones").unwrap_or("0");
    let semitones = semitones.parse::<i32>().unwrap();
    let input_tuning = matches.value_of("input-tuning").unwrap();
    let output_tuning = matches.value_of("output-tuning").unwrap();

    let keep_errors = matches.is_present("keep-errors");
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

    let mut style = Style::Default;
    if matches.is_present("style") {
        style = match matches.value_of("style").unwrap() {
            "b-bends" => Style::BBends,
            "harpsurgery" => Style::Harpsurgery,
            "plus" => Style::Plus,
            "draw" => Style::DrawDefault,
            _ => Style::Default,
        }
    }

    let play_audio = matches.is_present("play");

    let mut options = RunOptions {
        filename,
        semitones,
        from_position,
        to_position,
        octave_shift,
        keep_errors,
        style,
        input_tuning,
        output_tuning,
        _play_audio: play_audio,
        playable_positions: false,
        allow_bends: false,
    };

    if matches.is_present("playable-positions") {
        options.playable_positions = true;
        options.allow_bends = !matches.is_present("no-bends");
    }

    run(options);
}
