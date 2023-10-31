use crate::schema::address;

use diesel::prelude::{Queryable, Insertable};
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = address)]
pub struct Address {
    pub id: i32,
    pub flat_number: String,
    pub street: String,
    pub zip_code: String,
}