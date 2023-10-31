-- Your SQL goes here

CREATE TABLE TourBooking (
    booking_id INT PRIMARY KEY,
    customer_mail_id VARCHAR(255) NOT NULL,
    tour_id INT  NOT NULL,
    tour_booking_status VARCHAR(255) NOT NULL,
    booking_date DATE NOT NULL,
    FOREIGN KEY (customer_mail_id) REFERENCES Customer(mail_id),
    FOREIGN KEY (tour_id) REFERENCES Tour(id)
);
