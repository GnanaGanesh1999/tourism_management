use diesel::RunQueryDsl;
use rocket::{post, response::status::Created, serde::json::Json, get};

use crate::{
    models::tour_operator::TourOperator,
    schema,
    types::Result,
    utils::{establish_connection_pg, generate_id},
};

#[post("/", format = "json", data = "<tour_operator>")]
pub fn create_tour_operator(
    tour_operator: Json<TourOperator>,
) -> Result<Created<Json<TourOperator>>> {
    let connection = &mut establish_connection_pg();

    let new_tour_operator = TourOperator {
        mail_id: tour_operator.mail_id().to_string(),
        tour_operator_password: tour_operator.password().to_string(),
        tour_operator_name: tour_operator.name().to_string(),
    };

    match diesel::insert_into(schema::touroperator::table)
        .values(&new_tour_operator)
        .execute(connection)
    {
        Ok(_) => {
            println!("Tour Operator inserted");
            return Ok(Created::new("/").body(tour_operator));
        }
        Err(e) => {
            println!("Tour Operator cannot be inserted: {}", e);
            return Err(e.into());
        }
    }
}

#[get("/")]
pub fn get_tour_operators() -> Result<Json<Vec<TourOperator>>> {
    let connection = &mut establish_connection_pg();

    let tour_operators = schema::touroperator::table
        .load::<TourOperator>(connection)
        .expect("Error loading tour operators");

    Ok(Json(tour_operators))
}
