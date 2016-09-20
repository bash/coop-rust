mod coop;

extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::env;

use coop::commands::locations::show_locations;
use coop::commands::menus::show_menus;

const HELP: &'static str = "Usage: coop [COMMAND]

Available Subcommands:
 - menus [LOCATION]     -  Shows menus for a location
 - locations            -  Lists all available locations
";

fn main() {
    let command_wrapped = env::args().nth(1);

    if let Option::None = command_wrapped {
        println!("{}", HELP);
        return;
    }

    let command: String = command_wrapped.unwrap();

    match command.as_ref() {
        "menus" => show_menus(),
        "locations" => show_locations(),
        _ => println!("Unknown command {}", command)
    }
}