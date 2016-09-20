use std::io;
use std::io::Write;
use std::string::ToString;

#[derive(Copy, Clone)]
enum AnsiFormat {
    Reset = 0,
    Bold = 1,
    // Red = 31,
    Cyan = 36
}

impl ToString for AnsiFormat {
    fn to_string(&self) -> String {
        return format!("\x1b[{0}m", *self as u8);
    }
}

pub fn get_location() -> String {
    let mut input: String = String::new();

    print!("Location: ");

    io::stdout().flush().ok().expect("Failed to flush output");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    return input;
}

fn ansi_format(string: String, format: AnsiFormat) -> String {
    return format!("{}{}{}", format.to_string(), string, AnsiFormat::Reset.to_string());
}

pub fn bold(string: String) -> String {
    return ansi_format(string, AnsiFormat::Bold);
}

pub fn cyan(string: String) -> String {
    return ansi_format(string, AnsiFormat::Cyan);
}