extern crate hyper;

use hyper::client::Client;
use rustc_serialize::json;
use coop::menu::Response;

use std::string::String;
use std::io::Read;

const API_URL: &'static str = "https://themachine.jeremystucki.com/api/v1/coop/menus";

pub fn fetch_menus(timestamp: i64, location: String) -> Response {
    let client = Client::new();
    let url = format!("{}/{}/{}", API_URL, location, timestamp);

    let mut res = client.get(&url).send().unwrap();
    let mut string: String = String::new();

    res.read_to_string(&mut string);

    return json::decode(&string).unwrap();
}