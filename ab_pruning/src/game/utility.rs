use std::io::{self, Write};

pub fn read_user_input<T: std::str::FromStr>() -> T {
    io::stdout().flush().expect("flush failed!");

    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<T>() {
        Ok(i) => i,
        Err(..) => panic!("Invalid user input")
    }
}