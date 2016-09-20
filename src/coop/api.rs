extern crate hyper;

use std::string::String;
use std::io::Read;
use std::any::Any;

use hyper::client::Client;
use hyper::client::Response;
use rustc_serialize::json;

use coop::menu::Results;

const API_URL: &'static str = "https://themachine.jeremystucki.com/api/v1/coop/menus";

#[derive(Debug)]
pub enum ApiError {
    ParseError,
    FetchError
}

fn decode(res: &mut Response) -> Result<Results, ApiError> {
    let mut string = String::new();

    res.read_to_string(&mut string);

    let result = json::decode(&string);

    return match result {
        Err(err) => Err(ApiError::ParseError),
        Ok(value) => Ok(value)
    };
}

pub fn fetch_menus(timestamp: i64, location: String) -> Result<Results, ApiError> {
    let client = Client::new();
    let url = format!("{}/{}/{}", API_URL, location, timestamp);

    return match client.get(&url).send() {
        Ok(mut res) => decode(&mut res),
        Err(err) => Err(ApiError::FetchError)
    };
}