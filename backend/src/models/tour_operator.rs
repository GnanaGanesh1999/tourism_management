use diesel::prelude::{Insertable, Queryable};
use rocket::serde::{Deserialize, Serialize};

use crate::schema::touroperator;

#[diesel(table_name = touroperator)]
#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct TourOperator {
    pub mail_id: String,
    pub tour_operator_password: String,
    pub tour_operator_name: String,
}

impl TourOperator {
    pub fn new(
        mail_id: String,
        tour_operator_password: String,
        tour_operator_name: String,
    ) -> TourOperator {
        TourOperator {
            mail_id,
            tour_operator_password,
            tour_operator_name,
        }
    }

    pub fn mail_id(&self) -> &str {
        &self.mail_id
    }

    pub fn password(&self) -> &str {
        &self.tour_operator_password
    }

    pub fn name(&self) -> &str {
        &self.tour_operator_name
    }
}
