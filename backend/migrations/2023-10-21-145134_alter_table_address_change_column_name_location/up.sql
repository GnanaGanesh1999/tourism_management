-- Your SQL goes here
ALTER TABLE Address RENAME COLUMN zipcode TO zip_code;
ALTER TABLE Address DROP CONSTRAINT address_zipcode_fkey;
ALTER TABLE Address ADD CONSTRAINT address_zip_code_fkey FOREIGN KEY (zip_code) REFERENCES Zipcode(zip_code);