use std::io;
use std::io::Write;

pub fn get_location() -> String {
    let mut input: String = String::new();

    print!("Location: ");

    io::stdout().flush();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input;
}