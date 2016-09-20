use coop::data::Dishes;
use coop::api::fetch_dish_stats;

use coop::io::AnsiFormattable;

pub fn show_dish_stats() {
    let dishes: Dishes = fetch_dish_stats().unwrap();

    println!("Top Dishes:");

    for dish in dishes.results {
        println!("- {} {}", format!("{}x", dish.count).cyan(), dish.menu);
    }
}