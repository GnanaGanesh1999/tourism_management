-- Your SQL goes here
CREATE TABLE Address (
    id INT PRIMARY KEY,
    flat_number VARCHAR(255) NOT NULL,
    street VARCHAR(255) NOT NULL,
    zipcode INT NOT NULL,
    FOREIGN KEY (zipcode) REFERENCES Zipcode(zipcode)
);

