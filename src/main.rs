mod coop;

extern crate hyper;
extern crate rustc_serialize;
extern crate time;

use std::env;

use coop::time::midnight;
use coop::menu::Results;
use coop::api::fetch_menus;
use coop::io::get_location;

fn main() {
    let location_arg: Option<String> = env::args().nth(1);

    let location = match location_arg {
        Some(value) => value,
        None => get_location()
    };

    let response: Results = fetch_menus(midnight(), location).unwrap();

    for menu in response.results {
        println!("{0} {1}", menu.title, menu.price);

        for menu in menu.menu {
            println!(" - {}", menu);
        }

        println!("");
    }
}