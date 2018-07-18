#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
use rocket_contrib::Json;



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
    let events: Vec<Event> = vec![Event{
        date: "Sab 30 de Jun".to_string(),
        description: "Some cool string\n really nice".to_string(),
        id: 1,
        image_alt: "Image Alt 1".to_string(),
        image_url: "https://s3-us-west-2.amazonaws.com/pixel-solutions/event/banner/99e98ab25482f8eaae5742a9d94e91ed.jpg".to_string(),
        place: "Casa da Tia Rita".to_string(),
        price: 35.0,
        sales_place: "Venda: Bandejao UNICAMP, Casa da Tia Rita".to_string(),
        title: "Sample Title 1".to_string(),
    }];
    Json(events)
}

fn main() {
    rocket::ignite().mount("/", routes![get_events]).launch();
}