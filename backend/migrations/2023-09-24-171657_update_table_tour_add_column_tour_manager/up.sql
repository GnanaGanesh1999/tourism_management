-- Your SQL goes here

ALTER TABLE Tour
ADD tour_operator_id INT,
ADD FOREIGN KEY (tour_operator_id) REFERENCES TourOperator(id);
