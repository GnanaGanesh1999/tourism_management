-- Your SQL goes here
ALTER TABLE Address DROP CONSTRAINT address_zip_code_fkey;
ALTER TABLE Address ALTER COLUMN zip_code TYPE VARCHAR(6) USING zip_code::VARCHAR(6);
ALTER TABLE zipcode ALTER COLUMN zip_code TYPE VARCHAR(6);
ALTER TABLE Address ADD CONSTRAINT address_zip_code_fkey FOREIGN KEY (zip_code) REFERENCES Zipcode(zip_code);