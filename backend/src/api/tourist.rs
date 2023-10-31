use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use rocket::http::Status;
use rocket::response::status;
use rocket::response::{status::Created};
use rocket::serde::json::Json;
use rocket::{get, post};

use crate::types::Result;
use crate::models::tourist::{address::Address, customer::Customer, zipcode::ZipCode, Tourist};

use crate::schema::{self};
use crate::utils::establish_connection_pg;
use crate::utils::generate_id;
use crate::views::TouristView;



#[get("/")]
pub fn get_tourists() -> Result<status::Custom<Json<Vec<TouristView>>>> {
    use crate::schema::address::dsl::address;
    use crate::schema::customer::dsl::*;
    use crate::schema::zipcode::table;

    let connection = &mut establish_connection_pg();

    let results = customer
        .load::<Customer>(connection)
        .expect("Error loading customers");

    let mut tourists: Vec<TouristView> = Vec::new();

    for result in results {
        let address_result = address
            .filter(crate::schema::address::dsl::id.eq(result.address_id))
            .load::<Address>(connection)?;

        let address_ = address_result.get(0).unwrap();

        let zipcode_result = table
            .filter(crate::schema::zipcode::dsl::zip_code.eq(address_.zip_code.clone()))
            .load::<ZipCode>(connection)
            .expect("Error loading zipcode");

        let zipcode_ = zipcode_result.get(0).unwrap();

        let tourist = TouristView::new(
            result.mail_id,
            result.customer_name,
            address_.flat_number.clone(),
            address_.street.clone(),
            address_.zip_code.clone(),
            zipcode_.city.clone(),
            zipcode_.state_code.clone(),
            zipcode_.country.clone(),
        );

        tourists.push(tourist);
    }

    Ok(status::Custom(Status::Ok, Json(tourists)))
}

#[post("/", format = "json", data = "<tourist>")]
pub fn create_tourist(tourist: Json<Tourist>) -> Result<Created<Json<Tourist>>> {
    let connection = &mut establish_connection_pg();

    let address_id = generate_id();
    let customer_id = generate_id();

    let new_customer = Customer {
        mail_id: tourist.mail_id().to_string(),
        customer_password: tourist.password().to_string(),
        customer_name: tourist.name().to_string(),
        address_id: address_id,
    };

    let new_address = Address {
        id: address_id,
        flat_number: tourist.flat_number().to_string(),
        street: tourist.street().to_string(),
        zip_code: tourist.zip_code().to_string(),
    };

    let new_zipcode = ZipCode {
        zip_code: tourist.zip_code().to_string(),
        city: tourist.city().to_string(),
        state_code: tourist.state().to_string(),
        country: tourist.country().to_string(),
    };

    match diesel::insert_into(schema::zipcode::table)
        .values(&new_zipcode)
        .execute(connection)
    {
        Ok(_) => println!("Zipcode inserted"),
        Err(e) => println!("Zipcode cannot be inserted: {}", e),
    }

    match diesel::insert_into(schema::address::table)
        .values(&new_address)
        .execute(connection)
    {
        Ok(_) => println!("Address inserted"),
        Err(e) => println!("Address cannot be inserted: {}", e),
    }

    diesel::insert_into(schema::customer::table)
        .values(&new_customer)
        .execute(connection)
        .expect("Error saving new customer");

    Ok(Created::new("/").body(tourist))
}
