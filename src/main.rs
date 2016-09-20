mod coop;

extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::env;
use std::io;
use std::io::Write;

use coop::time::midnight;
use coop::menu::Response;
use coop::api::fetch_menus;

fn get_input() -> String {
    let mut input: String = String::new();

    print!("Location: ");

    io::stdout().flush();

    io::stdin()
      .read_line(&mut input)
      .expect("Failed to read input");

    return input;
}

fn main() {
    let location_arg: Option<String> = env::args().nth(1);

    let location = match location_arg {
        Some(value) => value,
        None => get_input()
    };

    let response: Response = fetch_menus(midnight(), location);

    for menu in response.results {
        println!("{0} {1}", menu.title, menu.price);

        for menu in menu.menu {
            println!(" - {}", menu);
        }

        println!("");
    }
}