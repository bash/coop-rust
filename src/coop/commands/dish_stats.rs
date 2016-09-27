use coop::data::Dishes;
use coop::api::{fetch_dish_stats_for_location, fetch_dish_stats, ApiError};
use coop::io::AnsiFormattable;

use std::env;

fn print_dishes(dishes: Dishes) {
    println!("Top Dishes:");

    for dish in dishes.results {
        println!("- {} {}", format!("{}x", dish.count).cyan(), dish.menu);
    }
}

pub fn show_dish_stats() -> Result<(), ApiError> {
    let dishes = match env::args().nth(2) {
        Some(location) => fetch_dish_stats_for_location(location),
        None => fetch_dish_stats()
    };

    match dishes {
        Ok(dishes) => print_dishes(dishes),
        Err(err) => return Err(err)
    };

    return Ok(());
}
