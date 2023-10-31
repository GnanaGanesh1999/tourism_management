
use crate::schema::customer;

use diesel::prelude::{Queryable, Insertable, Associations};
use rocket::response::Responder;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = customer)]
pub struct Customer {
    pub mail_id: String,
    pub customer_password: String,
    pub customer_name: String,
    pub address_id: i32,
}
