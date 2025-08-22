use std::io::{stdin, stdout, Write};
use termion::event::Key;
use rand::seq::{IndexedRandom};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use std::{thread, time};
use std::io;
use rand::{random_range, rng};

fn main() {
    let first_words = ["aquatic", "rhythmic", "pregnant", "digital", "diagonal", "cool", "interesting", "huge", "tiny", "medium",
    "plastic", "cautious", "sunless"];
    let second_words = ["soup", "sundress", "fishstick", "table", "keyboard", "workstation", "station", "particle", "dinosaur", "wormhole",
    "modification", "duck"];
    print!("Press [ENTER] to start.");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
    let mut enter_press = String::new();
    io::stdin()
        .read_line(&mut enter_press)
        .expect("Failed to read line");
    loop {
        let mut rng = rng();
        let word1 = first_words.choose(&mut rng).unwrap();
        let word2 = second_words.choose(&mut rng).unwrap();
        print!("Repository name: {}-{}", word1, word2);
        thread::sleep(time::Duration::from_millis(5));
        print!("\r                                                \r"); // overwrite with spaces, then return
    }
    print!("Okay boomer");
    std::io::Write::flush(&mut std::io::stdout()).unwrap();

    thread::sleep(time::Duration::from_secs(1));

    // Move cursor back and overwrite
    // print!("\r            \r"); // overwrite with spaces, then return
    // print!("Deleted text!");
}
