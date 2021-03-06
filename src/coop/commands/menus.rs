use std::env;
use std::process;

use coop::time::midnight;
use coop::data::Menus;
use coop::api::fetch_menus;
use coop::io::{get_location, AnsiFormattable};

pub fn show_menus() {
    let location_arg: Option<String> = env::args().nth(2);

    let location = match location_arg {
        Some(value) => value,
        None => get_location()
    };

    let response: Menus = fetch_menus(midnight(), &location).unwrap();

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