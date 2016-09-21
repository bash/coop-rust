use coop::data::Dishes;
use coop::api::fetch_dish_stats;
use coop::api::fetch_dish_stats_for_location;

use coop::io::AnsiFormattable;
use std::env;

pub fn show_dish_stats() {
    let dishes: Dishes = match env::args().nth(2) {
        Some(location) => fetch_dish_stats_for_location(location).unwrap(),
        None => fetch_dish_stats().unwrap()
    };

    println!("Top Dishes:");

    for dish in dishes.results {
        println!("- {} {}", format!("{}x", dish.count).cyan(), dish.menu);
    }
}