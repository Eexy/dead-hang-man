use std::{
    collections::HashSet,
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

fn obfuscate_word(tried_letters: &HashSet<char>, choosen_word: &str) -> String {
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

fn main() -> Result<(), String> {
    let dictionary = read_dictionary()?;

    let choosen_word_idx = rand::random_range(0..dictionary.len());
    let chosen_word = dictionary
        .get(choosen_word_idx)
        .ok_or("unable to select a word".to_string())?;

    let mut tried_letters: HashSet<char> = HashSet::new();
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

        let first_letter = buffer
            .trim()
            .chars()
            .next()
            .ok_or_else(|| "unable to extract char".to_string())?
            .to_ascii_uppercase();

        if tried_letters.insert(first_letter) {
            if !chosen_word.contains(first_letter) {
                tests += 1;
            }

            obfuscated_word = obfuscate_word(&tried_letters, &chosen_word);
        } else {
            println!("You already tried this letter: {}", first_letter);
        }
    }

    if chosen_word.eq(&obfuscated_word) {
        println!("you won");
    } else {
        print!("you lost");
    }

    Ok(())
}
