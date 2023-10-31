extern crate diesel;
extern crate rocket;
extern crate rocket_contrib;

use rocket::{launch, routes};

mod api;
mod models;
mod schema;
mod types;
mod utils;
mod views;

use api::tour_operator::{create_tour_operator, get_tour_operators};
use api::tourist::{create_tourist, get_tourists};
use api::tour::{create_tour, get_tours};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/tourist", routes![create_tourist, get_tourists])
        .mount("/tour-operator", routes![create_tour_operator, get_tour_operators])
        .mount("/tour", routes![create_tour, get_tours])
}
