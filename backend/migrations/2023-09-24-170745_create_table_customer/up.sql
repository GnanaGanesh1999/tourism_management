-- Your SQL goes here

CREATE TABLE Customer (
    mail_id VARCHAR(255) PRIMARY KEY,
    customer_password VARCHAR(255) NOT NULL,
    customer_name VARCHAR(255) NOT NULL,
    address_id INT NOT NULL,
    FOREIGN KEY (address_id) REFERENCES Address(id)
);

