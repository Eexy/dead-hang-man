use std::{
    collections::HashSet,
    fs::File,
    io::{self, BufRead, BufReader, Write, stdout},
    iter::repeat_n,
};

fn read_dictionary() -> Result<Vec<String>, String> {
    let file =
        File::open("dictionary.txt").map_err(|err| format!("unable to read dictionary: {err}"))?;
    let reader = BufReader::new(file);

    let lines = reader.lines().flatten().collect::<Vec<_>>();

    Ok(lines)
}

fn main() -> Result<(), String> {
    let dictionary = read_dictionary()?;
    dbg!(&dictionary);

    let choosen_word_idx = rand::random_range(0..dictionary.len());
    let choosen_word = &dictionary
        .get(choosen_word_idx)
        .ok_or("unable to select a word".to_string())?
        .to_ascii_uppercase();

    let mut tried_letters: HashSet<char> = HashSet::new();
    let mut buffer = String::new();
    let max_tries = 6;
    let mut tests = 0;
    let mut obfuscated_word = repeat_n('_', choosen_word.len()).collect::<String>();

    while !obfuscated_word.eq(choosen_word) && tests < max_tries {
        println!("word: {}", obfuscated_word);
        let tried_letters_stirng = tried_letters.iter().collect::<String>().replace("", " ");
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
            .chars()
            .next()
            .ok_or_else(|| "unable to extract char".to_string())?
            .to_ascii_uppercase();

        if !choosen_word.contains(first_letter) {
            tests += 1;
        }

        tried_letters.insert(first_letter);

        obfuscated_word = choosen_word
            .chars()
            .map(|ch| {
                if tried_letters.contains(&ch.to_ascii_uppercase()) {
                    return ch;
                }

                '_'
            })
            .collect::<String>();
    }

    if choosen_word.eq(&obfuscated_word) {
        println!("you won");
    } else {
        print!("you lost");
    }

    Ok(())
}
