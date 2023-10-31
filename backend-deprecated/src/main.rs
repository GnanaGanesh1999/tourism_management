mod schema;
use diesel::insert_into;

use diesel::prelude::*;
use std::error::Error;
use std::time::SystemTime;


use rocket::serde::{Deserialize, json::Json};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use schema::Address;

#[macro_use] extern crate rocket;


fn establish_connection() -> MysqlConnection {
    let database_url = "mysql://user:password@localhost/mydatabase";
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
#[diesel(table_name = "Address")]
struct NewAddress {
    flat_number: i32,
    street: String,
    zip_code: String,
}

#[post("/address", data = "<address>")]
fn address(address: Json<NewAddress>) -> String {
    let mysql_connection = establish_connection();
    let flat_number = address.flat_number;
    let street = address.street.clone();
    let zip_code = address.zip_code.clone();

    use schema::Address::dsl::*;
    diesel::insert_into(Address)
        .values((flat_number, street, zip_code))
        .execute(&mut &mysql_connection)
        .expect("Error saving new post");

    String::from("Inserted")
}
#[launch]
fn rocket() -> _ {
    rocket::build().mount("/insert", routes![address])
}