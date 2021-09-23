fn main() {
    let notes = ["1", "-1'", "-1", "1o", "2", "-2''", "-2'", "-2", "-3'''", "-3''", "-3'", "-3",
                 "4", "-4'", "-4", "4o", "5", "-5", "5o", "6", "-6'", "-6", "6o",
                 "-7", "7", "-7o", "-8", "8'", "8", "-9", "9'", "9", "-9o", "-10",
                 "10''", "10'", "10"];

    let tabs = ["-2", "-3", "4", "-4'", "-4", "-4"];

    for note in tabs {
        let mut pos = notes.iter().position(|&x| x == note).unwrap() as i32;
        pos += -7;
        let new_note = notes.get(pos as usize).unwrap();
        print!("{} ", new_note);
    }
    println!();
}
