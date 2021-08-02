#[macro_use]
extern crate rocket;

use rand::{thread_rng, Rng};
use rocket::{
    fs::FileServer,
    serde::{json::Json, Deserialize, Serialize},
    State,
};
use std::fs::File;

#[derive(Serialize, Deserialize)]
struct Quote {
    quote: String,
    by: String,
}

#[get("/quote")]
fn quote(quotes: &State<Vec<Quote>>) -> Json<&Quote> {
    let index: usize = thread_rng().gen_range(0..quotes.len());
    let quote = &quotes[index];
    Json(quote)
}

#[launch]
fn rocket() -> _ {
    let file = File::open("unspirational.json").unwrap();
    let quotes: Vec<Quote> = serde_json::from_reader(file).unwrap();
    rocket::build()
        .manage(quotes)
        .mount("/", FileServer::from("static"))
        .mount("/", routes![quote])
}
