use clap::{App, Arg};
use harptabs::run;

fn main() {
    let matches = App::new("harptool")
        .about("print harmonica note layouts")
        .arg(
            Arg::with_name("semitones")
                .short("s")
                .long("semitones")
                .value_name("SEMITONES")
                .allow_hyphen_values(true)
                .help("number of semitones to transpose"),
        )
        .arg(
            Arg::with_name("file")
                .value_name("FILE")
                .help("file containing tabs"),
        )
        .get_matches();

    let filename = matches.value_of("file").unwrap_or("tabs.txt");
    let semitones = matches.value_of("semitones").unwrap_or("0");
    let semitones = semitones.parse::<i32>().unwrap();

    run(filename, semitones);
}
