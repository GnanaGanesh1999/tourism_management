-- Your SQL goes here
CREATE TABLE Tour (
    id INT PRIMARY KEY,
    capacity INT,
    cost DECIMAL,
    number_of_days INT
);

CREATE TABLE Location (
    id INT PRIMARY KEY,
    location VARCHAR(255),
    FOREIGN KEY (id) REFERENCES Tour(id)
);

