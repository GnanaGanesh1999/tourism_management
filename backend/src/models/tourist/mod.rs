pub mod address;
pub mod customer;
pub mod zipcode;

use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Tourist {
    mail_id: String,
    password: String,
    name: String,
    flat_number: String,
    street: String,
    zip_code: String,
    city: String,
    state: String,
    country: String,
}

impl Tourist {
    pub fn new(
        mail_id: String,
        password: String,
        name: String,
        flat_number: String,
        street: String,
        zip_code: String,
        city: String,
        state: String,
        country: String,
    ) -> Tourist {
        Tourist {
            mail_id,
            password,
            name,
            flat_number,
            street,
            zip_code,
            city,
            state,
            country,
        }
    }

    pub fn mail_id(&self) -> &str {
        &self.mail_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn flat_number(&self) -> &str {
        &self.flat_number
    }

    pub fn street(&self) -> &str {
        &self.street
    }

    pub fn zip_code(&self) -> &str {
        &self.zip_code
    }

    pub fn city(&self) -> &str {
        &self.city
    }

    pub fn state(&self) -> &str {
        &self.state
    }

    pub fn country(&self) -> &str {
        &self.country
    }

    
}