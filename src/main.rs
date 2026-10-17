use std::io::{self};

fn main() -> Result<(), String> {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .map_err(|err| format!("unable to read user input: {err}"))?;
    let cleaned_user_input = buffer.trim();
    dbg!(cleaned_user_input);
    Ok(())
}
