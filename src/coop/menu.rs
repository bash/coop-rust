#[derive(RustcDecodable)]
pub struct Menu {
    pub price: f32,
    pub timestap: f32,
    pub title: String,
    pub menu: Vec<String>
}

#[derive(RustcDecodable)]
pub struct Response {
    pub results: Vec<Menu>
}