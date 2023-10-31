// @generated automatically by Diesel CLI.

diesel::table! {
    address (id) {
        id -> Int4,
        #[max_length = 255]
        flat_number -> Varchar,
        #[max_length = 255]
        street -> Varchar,
        #[max_length = 6]
        zip_code -> Varchar,
    }
}

diesel::table! {
    customer (mail_id) {
        #[max_length = 255]
        mail_id -> Varchar,
        #[max_length = 255]
        customer_password -> Varchar,
        #[max_length = 255]
        customer_name -> Varchar,
        address_id -> Int4,
    }
}

diesel::table! {
    location (id) {
        id -> Int4,
        #[max_length = 255]
        location_name -> Varchar,
        #[max_length = 255]
        location_description -> Varchar,
    }
}

diesel::table! {
    tour (id) {
        id -> Int4,
        #[max_length = 255]
        capacity -> Varchar,
        #[max_length = 255]
        cost -> Varchar,
        #[max_length = 255]
        number_of_days -> Varchar,
        #[max_length = 255]
        tour_start_date -> Varchar,
        #[max_length = 255]
        end_date -> Varchar,
        #[max_length = 255]
        tour_name -> Varchar,
        #[max_length = 255]
        tour_description -> Varchar,
        #[max_length = 255]
        tour_operator_id -> Varchar,
    }
}

diesel::table! {
    tourbooking (booking_id) {
        booking_id -> Int4,
        #[max_length = 255]
        customer_mail_id -> Varchar,
        tour_id -> Int4,
        #[max_length = 255]
        tour_booking_status -> Varchar,
        booking_date -> Date,
    }
}

diesel::table! {
    touroperator (mail_id) {
        #[max_length = 255]
        mail_id -> Varchar,
        #[max_length = 255]
        tour_operator_password -> Varchar,
        #[max_length = 255]
        tour_operator_name -> Varchar,
    }
}

diesel::table! {
    zipcode (zip_code) {
        #[max_length = 6]
        zip_code -> Varchar,
        #[max_length = 255]
        state_code -> Varchar,
        #[max_length = 255]
        city -> Varchar,
        #[max_length = 255]
        country -> Varchar,
    }
}

diesel::joinable!(address -> zipcode (zip_code));
diesel::joinable!(customer -> address (address_id));
diesel::joinable!(location -> tour (id));
diesel::joinable!(tour -> touroperator (tour_operator_id));
diesel::joinable!(tourbooking -> customer (customer_mail_id));
diesel::joinable!(tourbooking -> tour (tour_id));

diesel::allow_tables_to_appear_in_same_query!(
    address,
    customer,
    location,
    tour,
    tourbooking,
    touroperator,
    zipcode,
);
