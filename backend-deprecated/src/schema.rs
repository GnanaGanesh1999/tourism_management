// @generated automatically by Diesel CLI.

diesel::table! {
    Address (id) {
        id -> Integer,
        #[max_length = 255]
        flat_number -> Nullable<Varchar>,
        #[max_length = 255]
        street -> Nullable<Varchar>,
        zipcode -> Nullable<Integer>,
    }
}

diesel::table! {
    Customer (id) {
        id -> Integer,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
        address_id -> Nullable<Integer>,
    }
}

diesel::table! {
    Location (id) {
        id -> Integer,
        #[max_length = 255]
        location -> Nullable<Varchar>,
    }
}

diesel::table! {
    Tour (id) {
        id -> Integer,
        capacity -> Nullable<Integer>,
        cost -> Nullable<Decimal>,
        number_of_days -> Nullable<Integer>,
        tour_operator_id -> Nullable<Integer>,
    }
}

diesel::table! {
    TourBooking (booking_id) {
        booking_id -> Integer,
        customer_id -> Nullable<Integer>,
        tour_id -> Nullable<Integer>,
        #[max_length = 255]
        status -> Nullable<Varchar>,
    }
}

diesel::table! {
    TourOperator (id) {
        id -> Integer,
        #[max_length = 255]
        mail_id -> Nullable<Varchar>,
        #[max_length = 255]
        password -> Nullable<Varchar>,
        #[max_length = 255]
        name -> Nullable<Varchar>,
    }
}

diesel::table! {
    Zipcode (zipcode) {
        zipcode -> Integer,
        #[max_length = 255]
        state -> Nullable<Varchar>,
        #[max_length = 255]
        city -> Nullable<Varchar>,
    }
}

diesel::table! {
    sys_config (variable) {
        #[max_length = 128]
        variable -> Varchar,
        #[max_length = 128]
        value -> Nullable<Varchar>,
        set_time -> Nullable<Timestamp>,
        #[max_length = 128]
        set_by -> Nullable<Varchar>,
    }
}

diesel::joinable!(Address -> Zipcode (zipcode));
diesel::joinable!(Customer -> Address (address_id));
diesel::joinable!(Location -> Tour (id));
diesel::joinable!(Tour -> TourOperator (tour_operator_id));
diesel::joinable!(TourBooking -> Customer (customer_id));
diesel::joinable!(TourBooking -> Tour (tour_id));

diesel::allow_tables_to_appear_in_same_query!(
    Address,
    Customer,
    Location,
    Tour,
    TourBooking,
    TourOperator,
    Zipcode,
    sys_config,
);
