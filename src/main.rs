mod coop;

extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::env;

use coop::time::midnight;
use coop::menu::Results;
use coop::api::fetch_menus;
use coop::io::{get_location, cyan, bold};

fn main() {
    let location_arg: Option<String> = env::args().nth(1);

    let location = match location_arg {
        Some(value) => value,
        None => get_location()
    };

    let response: Results = fetch_menus(midnight(), location).unwrap();

    for menu in response.results {
        println!("\n{0} {1}", bold(menu.title), cyan(menu.price.to_string()));

        for menu in menu.menu {
            println!(" - {}", menu);
        }
    }

    println!("");
}