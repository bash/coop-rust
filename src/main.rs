mod coop;

extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::env;

use coop::commands::locations::show_locations;
use coop::commands::menus::show_menus;
use coop::commands::dish_stats::show_dish_stats;

const HELP: &'static str = "Usage: coop [COMMAND]

Available Subcommands:
 - menus [LOCATION]       -  Shows menus for a location
 - locations              -  Lists all available locations
 - dish-stats [LOCATION]  -  Shows stats dishes. Optionally only for one location.
";

fn help() {
    println!("{}", HELP);
}

fn main() {
    let command_wrapped = env::args().nth(1);

    if let Option::None = command_wrapped {
        help();
        return;
    }

    let command: String = command_wrapped.unwrap();

    match command.as_ref() {
        "menus" => show_menus(),
        "locations" => show_locations(),
        "dish-stats" => show_dish_stats().unwrap(),
        "--help" => help(),
        _ => println!("Unknown command {}", command)
    }
}