use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

use rand::seq::IndexedRandom;

fn read_dictionary() -> Result<Vec<String>, String> {
    let file =
        File::open("dictionary.txt").map_err(|err| format!("unable to read dictionary: {err}"))?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .filter_map(|line| line.map(|l| l.trim().to_ascii_uppercase()).ok())
        .collect::<Vec<_>>();

    Ok(lines)
}

fn obfuscate_word(tried_letters: &BTreeSet<char>, choosen_word: &str) -> String {
    choosen_word
        .chars()
        .map(|ch| {
            if tried_letters.contains(&ch) {
                return ch;
            }

            '_'
        })
        .collect::<String>()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Outcome {
    Won,
    Lost,
}

impl std::fmt::Display for Outcome {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Outcome::Won => "won",
            Outcome::Lost => "lost",
        })
    }
}

const SAVES_PATH: &str = "saves.txt";

fn load_saves() -> Result<File, String> {
    File::options()
        .append(true)
        .create(true)
        .open(SAVES_PATH)
        .map_err(|err| format!("unable to open create / open saves file: {err}"))
}

fn save_game(
    chosen_word: &str,
    number_of_failed_tries: u8,
    outcome: Outcome,
) -> Result<(), String> {
    let mut save_file = load_saves()?;
    let content = format!(
        "word to find: {chosen_word}, number of failed tries: {number_of_failed_tries}, status: {outcome};\n"
    );

    save_file
        .write_all(content.as_bytes())
        .map_err(|err| format!("unable to write save file: {err}"))?;
    Ok(())
}

fn main() -> Result<(), String> {
    let dictionary = read_dictionary()?;

    let chosen_word = dictionary
        .choose(&mut rand::rng())
        .ok_or("unable to select a word")?;

    let mut tried_letters: BTreeSet<char> = BTreeSet::new();
    let mut buffer = String::new();
    let max_tries = 6u8;
    let mut tests = 0u8;
    let mut obfuscated_word = obfuscate_word(&tried_letters, chosen_word);

    while !obfuscated_word.eq(chosen_word) && tests < max_tries {
        println!("word: {obfuscated_word}");
        let tried_letters_stirng = tried_letters
            .iter()
            .map(char::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        println!("Previous tried letters: {tried_letters_stirng}");

        print!("Please enter a letter: ");
        io::stdout()
            .flush()
            .map_err(|err| format!("unable to flush stdout: {err}"))?;
        buffer.clear();
        io::stdin()
            .read_line(&mut buffer)
            .map_err(|err| format!("unable to read user input: {err}"))?;

        let Some(letter) = buffer.trim().chars().next() else {
            println!("Please type a letter");
            continue;
        };

        if !letter.is_ascii_alphabetic() {
            println!("'{letter}' is not a letter.");
            continue;
        }

        let uppercased_letter = letter.to_ascii_uppercase();

        if !tried_letters.insert(uppercased_letter) {
            println!("You already tried this letter: {uppercased_letter}");
            continue;
        }

        if !chosen_word.contains(uppercased_letter) {
            tests += 1;
        }

        obfuscated_word = obfuscate_word(&tried_letters, chosen_word);
    }

    let outcome = if chosen_word.eq(&obfuscated_word) {
        Outcome::Won
    } else {
        Outcome::Lost
    };

    println!("you {outcome}, chosen word were: {chosen_word}");

    save_game(chosen_word, tests, outcome)?;

    Ok(())
}
