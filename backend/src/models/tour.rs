

use diesel::{prelude::{Queryable, Insertable}, sql_types::{Date, Integer}, expression::AsExpression};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::tour;

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = tour)]
pub struct Tour {
    pub id: i32,
    pub capacity: String,
    pub cost: String,
    pub number_of_days: String,
    pub tour_operator_id: String,
    pub tour_name: String,
    pub tour_description: String,
    pub tour_start_date: String,
    pub end_date: String
}

