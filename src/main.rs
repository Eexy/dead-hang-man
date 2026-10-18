use std::{
    collections::BTreeSet,
    fs::File,
    io::{self, BufRead, BufReader, Write},
};

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

fn load_saves() -> Result<File, String> {
    Ok(File::options()
        .append(true)
        .create(true)
        .open("saves.txt")
        .map_err(|err| format!("unable to open create / open saves file: {err}"))?)
}

fn save_game(chosen_word: &str, number_of_tries: i32, is_won: bool) -> Result<(), String> {
    let mut save_file = load_saves()?;
    let status = if is_won { "won" } else { "lost" };
    let content = format!(
        "word to find: {chosen_word}, number of tries: {number_of_tries}, status: {status};"
    );
    writeln!(&mut save_file, "{content}")
        .map_err(|err| format!("unable to write to save file {err}"))?;

    Ok(())
}

fn main() -> Result<(), String> {
    let dictionary = read_dictionary()?;

    let choosen_word_idx = rand::random_range(0..dictionary.len());
    let chosen_word = dictionary
        .get(choosen_word_idx)
        .ok_or("unable to select a word".to_string())?;

    let mut tried_letters: BTreeSet<char> = BTreeSet::new();
    let mut buffer = String::new();
    let max_tries = 6;
    let mut tests = 0;
    let mut obfuscated_word = obfuscate_word(&tried_letters, chosen_word);

    while !obfuscated_word.eq(chosen_word) && tests < max_tries {
        println!("word: {}", obfuscated_word);
        let tried_letters_stirng = tried_letters
            .iter()
            .map(|ch| ch.to_string())
            .collect::<Vec<_>>()
            .join(" ");
        println!("Previous tried letters: {}", tried_letters_stirng);

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

        obfuscated_word = obfuscate_word(&tried_letters, &chosen_word);
    }

    let is_won = chosen_word.eq(&obfuscated_word);

    if is_won {
        println!("you won");
    } else {
        print!("you lost");
    }

    save_game(chosen_word, tests, is_won)?;

    Ok(())
}
