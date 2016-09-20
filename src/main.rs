mod coop;

extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::env;
use std::process;

use coop::time::midnight;
use coop::menu::Results;
use coop::locations::Locations;
use coop::api::{fetch_menus, fetch_locations};
use coop::io::{get_location, AnsiFormattable};

const HELP: &'static str = "Usage: coop [COMMAND]

Available Subcommands:
 - menus [LOCATION]     -  Shows menus for a location
 - locations            -  Lists all available locations
";

fn menus() {
    let location_arg: Option<String> = env::args().nth(2);

    let location = match location_arg {
        Some(value) => value,
        None => get_location()
    };

    let response: Results = fetch_menus(midnight(), &location).unwrap();

    if response.results.len() == 0 {
        print!("{}", format!("No menus found for {}", location).red());
        process::exit(1);
    }

    for menu in response.results {
        println!("\n{0} {1}", menu.title.bold(), menu.price.to_string().cyan());

        for menu in menu.menu {
            println!(" - {}", menu);
        }
    }

    println!("");
}

fn locations() {
    let response: Locations = fetch_locations().unwrap();

    println!("Available locations");

    for location in response.results {
        println!(" - {}", location);
    }
}

fn help() {
    println!("{}", HELP);
}

fn unknown(command: String) {
    println!("Unknown command {}", command);
}

fn main() {
    let command_wrapped = env::args().nth(1);

    if let Option::None = command_wrapped {
        help();
        return;
    }

    let command: String = command_wrapped.unwrap();

    match command.as_ref() {
        "menus" => menus(),
        "locations" => locations(),
        _ => unknown(command)
    }
}