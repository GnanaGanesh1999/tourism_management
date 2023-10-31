-- Your SQL goes here

CREATE TABLE Zipcode (
    zipcode INT PRIMARY KEY,
    state_code VARCHAR(255) NOT NULL,
    city VARCHAR(255) NOT NULL,
    country VARCHAR(255) NOT NULL
);
