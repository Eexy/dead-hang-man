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
        .filter_map(|line| line.map(|l| l.trim().to_string()).ok())
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    Ok(lines)
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

fn open_saves_file() -> Result<File, String> {
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
    let mut save_file = open_saves_file()?;

    writeln!(
        save_file,
        "word to find: {chosen_word}, number of failed tries: {number_of_failed_tries}, status: {outcome};"
    )
    .map_err(|err| format!("unable to write save file: {err}"))
}

enum GuessResult {
    Hit,
    Miss,
    AlreadyTried,
    InvalidInput,
}

struct Game {
    word: String,
    letter_to_find: BTreeSet<char>,
    tried: BTreeSet<char>,
    failed_tries: u8,
    max_tries: u8,
}

impl Game {
    fn new(word: &str, max_tries: u8) -> Self {
        let uppercased_word = word.to_ascii_uppercase();

        Self {
            letter_to_find: uppercased_word.chars().collect(),
            word: uppercased_word,
            tried: BTreeSet::new(),
            failed_tries: 0,
            max_tries,
        }
    }

    fn is_won(&self) -> bool {
        self.letter_to_find.is_subset(&self.tried)
    }

    fn is_lost(&self) -> bool {
        self.failed_tries >= self.max_tries
    }

    fn masked(&self) -> String {
        self.word
            .chars()
            .map(|ch| if self.tried.contains(&ch) { ch } else { '_' })
            .collect()
    }

    fn guess(&mut self, letter: char) -> GuessResult {
        if !letter.is_ascii_alphabetic() {
            return GuessResult::InvalidInput;
        }

        let letter = letter.to_ascii_uppercase();
        // Insert letter and return a boolean. true when letter doesn't exist and false otherwise
        if !self.tried.insert(letter) {
            return GuessResult::AlreadyTried;
        }

        if !self.letter_to_find.contains(&letter) {
            self.failed_tries += 1;
            return GuessResult::Miss;
        }

        GuessResult::Hit
    }
}

const ALLOWED_MAX_TRIES: u8 = 6;

fn main() -> Result<(), String> {
    let dictionary = read_dictionary()?;

    let chosen_word = dictionary
        .choose(&mut rand::rng())
        .ok_or("unable to select a word")?;

    let mut game = Game::new(chosen_word, ALLOWED_MAX_TRIES);

    let mut buffer = String::new();

    while !game.is_lost() && !game.is_won() {
        println!("word: {}", game.masked());
        let tried_letters_string = game
            .tried
            .iter()
            .map(char::to_string)
            .collect::<Vec<_>>()
            .join(" ");
        println!("Previous tried letters: {tried_letters_string}");

        print!("Please enter a letter: ");
        io::stdout()
            .flush()
            .map_err(|err| format!("unable to flush stdout: {err}"))?;
        buffer.clear();
        let bytes_read = io::stdin()
            .read_line(&mut buffer)
            .map_err(|err| format!("unable to read user input: {err}"))?;

        if bytes_read == 0 {
            return Err("input closed".into());
        }

        let Some(letter) = buffer.trim().chars().next() else {
            println!("Please type a letter");
            continue;
        };

        match game.guess(letter) {
            GuessResult::InvalidInput => {
                println!("'{letter}' is not a letter.");
            }
            GuessResult::AlreadyTried => {
                println!("You already tried this letter: {letter}");
            }
            GuessResult::Miss => {
                println!("wrong guess");
            }
            GuessResult::Hit => {}
        }
    }

    let outcome = if game.is_won() {
        Outcome::Won
    } else {
        Outcome::Lost
    };

    println!("you {outcome}, chosen word were: {}", game.word);

    save_game(&game.word, game.failed_tries, outcome)?;

    Ok(())
}
