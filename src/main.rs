#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
extern crate rocket_cors;

use rocket_contrib::json::Json;
use rocket_cors::{AllowedHeaders, AllowedOrigins};
use rocket::http::Method;
use rocket::Config;
use rocket::config::Environment;
use std::error::Error;


#[derive(Serialize)]
struct Event {
    date: String,
    description: String,
    id: u64,
    image_alt: String,
    image_url: String,
    place: String,
    price: f32,
    sales_place: String,
    title: String
}

#[get("/api/events")]
fn get_events() -> Json<Vec<Event>> {
    let events: Vec<Event> = vec![
        Event{
            date: "Sab 30 de Jun".to_string(),
            description: "Some cool string\n really nice".to_string(),
            id: 1,
            image_alt: "Image Alt 1".to_string(),
            image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg".to_string(),
            place: "Casa da Tia Rita".to_string(),
            price: 35.0,
            sales_place: "Venda: Bandejao UNICAMP, Casa da Tia Rita".to_string(),
            title: "Sample Title 1".to_string(),
        },
        Event{
            date: "Sab 10 de Jun".to_string(),
            description: "Some loren, really nice".to_string(),
            id: 2,
            image_alt: "Image Alt 2".to_string(),
            image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg".to_string(),
            place: "Segunda Casa da Tia Rita".to_string(),
            price: 50.0,
            sales_place: "Venda: Bandejao UNICAMP, Casa da Tia Rita".to_string(),
            title: "Sample Title 2".to_string(),
        },
        Event{
            date: "Sab 30 de Jun".to_string(),
            description: "Some cool string\n really nice".to_string(),
            id: 3,
            image_alt: "Image Alt 1".to_string(),
            image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg".to_string(),
            place: "Casa da Tia Rita".to_string(),
            price: 35.0,
            sales_place: "Venda: Bandejao UNICAMP, Casa da Tia Rita".to_string(),
            title: "Sample Title 1".to_string(),
        },
        Event{
            date: "Sab 10 de Jun".to_string(),
            description: "Some loren, really nice".to_string(),
            id: 4,
            image_alt: "Image Alt 2".to_string(),
            image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg".to_string(),
            place: "Segunda Casa da Tia Rita".to_string(),
            price: 50.0,
            sales_place: "Venda: Bandejao UNICAMP, Casa da Tia Rita".to_string(),
            title: "Sample Title 2".to_string(),
        }];
    Json(events)
}



fn main() -> Result<(), Box<Error>> {
    let (allowed_origins, failed_origins) = AllowedOrigins::some(&["http://10.0.0.12:3000", "http://127.0.0.1:8000", "http://localhost:8000", "http://localhost:3000"]);

    assert!(failed_origins.is_empty());

// You can also deserialize this
    let options = rocket_cors::Cors {
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    };
    let mut config = Config::new(Environment::Development);
    config.set_port(1024);

    rocket::custom(config).mount("/", routes![get_events])
        .attach(options)
        .launch();

    Ok(())
}