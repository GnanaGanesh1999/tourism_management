use diesel::RunQueryDsl;
use rocket::get;
use rocket::response::status::Created;
use rocket::{post, serde::json::Json};

use crate::models::tour::{Tour, self};
use crate::models::tour_operator::TourOperator;
use crate::schema;
use crate::types::Result;
use crate::utils::{generate_id, establish_connection_pg};
use crate::views::tour::TourView;


#[post("/", format = "json", data = "<tour>")]
pub fn create_tour(tour: Json<TourView>) -> Result<Created<Json<TourView>>> {

    let connection = &mut establish_connection_pg();


    let new_tour = Tour {
        id: generate_id(),
        capacity: tour.capacity.clone(),
        cost: tour.cost.clone(),
        number_of_days: tour.number_of_days.clone(),
        tour_operator_id: tour.tour_operator_id.clone().unwrap(),
        tour_name: tour.tour_name.to_string(),
        tour_description: tour.tour_description.to_string(),
        tour_start_date: tour.tour_start_date.to_string(),
        end_date: tour.end_date.to_string(),
    };

    let tour_operator = TourOperator {
        mail_id: tour.tour_operator_id.clone().unwrap().clone(),
        tour_operator_password: tour.tour_operator_password.clone().unwrap().clone(),
        tour_operator_name: tour.tour_operator_name.clone().unwrap().clone(),
    };
    // TODO: Check if tour operator exists
    match diesel::insert_into(schema::touroperator::table)
        .values(&tour_operator)
        .execute(connection) {
        Ok(_) => {
            println!("Tour Operator inserted");
        },

        Err(e) => {
            println!("Tour Operator cannot be inserted: {}", e);
         
        }
    }

    diesel::insert_into(schema::tour::table)
        .values(&new_tour)
        .execute(connection)?;

    let tour_view = TourView::from(new_tour);
    Ok(Created::new("/").body(Json(tour_view)))
}

#[get("/")]
pub fn get_tours() -> Result<Json<Vec<Tour>>> {
    let connection = &mut establish_connection_pg();

    let tours = schema::tour::table
        .load::<Tour>(connection)
        .expect("Error loading tours");

    Ok(Json(tours))
}