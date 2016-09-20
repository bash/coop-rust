extern crate hyper;

use std::string::String;
use std::io::Read;

use hyper::client::{Client, Response};
use rustc_serialize::{json, Decodable};

use coop::data::{Results, Locations};

const MENUS_ENDPOINT: &'static str = "https://themachine.jeremystucki.com/api/v1/coop/menus";
const LOCATIONS_ENDPOINT: &'static str = "https://themachine.jeremystucki.com/api/v1/coop/locations";

#[derive(Debug)]
pub enum ApiError {
    ParseError,
    FetchError
}

fn decode<T: Decodable>(res: &mut Response) -> Result<T, ApiError> {
    let mut string = String::new();
    let readres = res.read_to_string(&mut string);

    if let Err(_) = readres {
        return Err(ApiError::FetchError);
    }

    let result: json::DecodeResult<T> = json::decode(&string);

    return match result {
        Ok(value) => Ok(value),
        Err(_) => Err(ApiError::ParseError)
    };
}

fn fetch<T: Decodable>(url: String) -> Result<T, ApiError> {
    let client = Client::new();

    return match client.get(&url).send() {
        Ok(mut res) => decode(&mut res),
        Err(_) => Err(ApiError::FetchError)
    }
}

pub fn fetch_menus(timestamp: i64, location: &String) -> Result<Results, ApiError> {
    return fetch(format!("{}/{}/{}", MENUS_ENDPOINT, location, timestamp));
}

pub fn fetch_locations() -> Result<Locations, ApiError> {
    return fetch(LOCATIONS_ENDPOINT.to_string());
}