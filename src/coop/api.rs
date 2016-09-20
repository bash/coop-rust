extern crate hyper;

use std::string::String;
use std::io::Read;

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
    let readres = res.read_to_string(&mut string);

    if let Err(_) = readres {
        return Err(ApiError::FetchError);
    }

    let result = json::decode(&string);

    return match result {
        Ok(value) => Ok(value),
        Err(_) => Err(ApiError::ParseError)
    };
}

pub fn fetch_menus(timestamp: i64, location: &String) -> Result<Results, ApiError> {
    let client = Client::new();
    let url = format!("{}/{}/{}", API_URL, location, timestamp);

    let resp_wrapped = client.get(&url).send();

    if let Ok(mut res) = resp_wrapped {
        return decode(&mut res);
    }

    return Err(ApiError::FetchError);
}