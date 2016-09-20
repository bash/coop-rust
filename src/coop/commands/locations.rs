use coop::data::Locations;
use coop::api::fetch_locations;

pub fn show_locations() {
    let response: Locations = fetch_locations().unwrap();

    println!("Available locations");

    for location in response.results {
        println!(" - {}", location);
    }
}