-- Your SQL goes here

ALTER TABLE Tour
ADD tour_operator_id VARCHAR(255) NOT NULL,
ADD FOREIGN KEY (tour_operator_id) REFERENCES TourOperator(mail_id);
