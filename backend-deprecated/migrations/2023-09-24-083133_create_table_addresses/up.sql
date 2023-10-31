-- Your SQL goes here
CREATE TABLE Address (
    id INT PRIMARY KEY,
    flat_number VARCHAR(255),
    street VARCHAR(255),
    zipcode INT,
    FOREIGN KEY (zipcode) REFERENCES Zipcode(zipcode)
);

