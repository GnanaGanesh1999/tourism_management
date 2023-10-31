-- Your SQL goes here

CREATE TABLE TourBooking (
    booking_id INT PRIMARY KEY,
    customer_id INT,
    tour_id INT,
    status VARCHAR(255),
    FOREIGN KEY (customer_id) REFERENCES Customer(id),
    FOREIGN KEY (tour_id) REFERENCES Tour(id)
);
