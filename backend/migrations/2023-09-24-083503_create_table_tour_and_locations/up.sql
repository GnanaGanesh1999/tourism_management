-- Your SQL goes here
CREATE TABLE Tour (
    id INT PRIMARY KEY,
    capacity VARCHAR(255) NOT NULL,
    cost VARCHAR(255) NOT NULL,
    number_of_days VARCHAR(255) NOT NULL,
    tour_start_date VARCHAR(255) NOT NULL,
    end_date VARCHAR(255) NOT NULL,
    tour_name VARCHAR(255) NOT NULL,
    tour_description VARCHAR(255) NOT NULL
);

CREATE TABLE Location (
    id INT PRIMARY KEY,
    location_name VARCHAR(255) NOT NULL,
    location_description VARCHAR(255) NOT NULL,
    FOREIGN KEY (id) REFERENCES Tour(id)
);

