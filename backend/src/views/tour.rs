use rocket::serde::{Deserialize, Serialize};

use crate::{models::tour::Tour, schema::tour};

#[derive(Serialize, Deserialize)]
pub struct TourView {
    pub capacity: String,
    pub cost: String,
    pub number_of_days: String,
    pub tour_operator_id: Option<String>,
    pub tour_operator_password: Option<String>,
    pub tour_operator_name: Option<String>,
    pub tour_name: String,
    pub tour_description: String,
    pub tour_start_date: String,
    pub end_date: String,
}

impl TourView {
    pub fn new(
        capacity: String,
        cost: String,
        number_of_days: String,
        mail_id: String,
        tour_operator_password: String,
        tour_operator_name: String,
        tour_name: String,
        tour_description: String,
        tour_start_date: String,
        end_date: String,
    ) -> TourView {
        TourView {
            capacity,
            cost,
            number_of_days,
            tour_operator_id: Some(mail_id),
            tour_operator_password: Some(tour_operator_password),
            tour_operator_name: Some(tour_operator_name),
            tour_name,
            tour_description,
            tour_start_date,
            end_date,
        }
    }
}

impl From<Tour> for TourView {
    fn from(tour: Tour) -> Self {
        TourView {
            capacity: tour.capacity,
            cost: tour.cost,
            number_of_days: tour.number_of_days,
            tour_operator_id: None,
            tour_operator_password: None,
            tour_operator_name: None,
            tour_name: tour.tour_name,
            tour_description: tour.tour_description,
            tour_start_date: tour.tour_start_date,
            end_date: tour.end_date,
        }
    }
}