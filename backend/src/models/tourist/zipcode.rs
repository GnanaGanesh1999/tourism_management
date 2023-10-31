use crate::schema::zipcode;

use serde::{Serialize, Deserialize};
use diesel::prelude::{Queryable, Insertable};

#[derive(Queryable, Insertable, Serialize, Deserialize)]  
#[diesel(table_name = zipcode)]
pub struct ZipCode {
    pub zip_code: String,
    pub city: String,
    pub state_code: String,
    pub country: String,
}
