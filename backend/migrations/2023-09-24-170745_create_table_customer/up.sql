-- Your SQL goes here

CREATE TABLE Customer (
    id INT PRIMARY KEY,
    mail_id VARCHAR(255),
    password VARCHAR(255),
    name VARCHAR(255),
    address_id INT,
    FOREIGN KEY (address_id) REFERENCES Address(id)
);

