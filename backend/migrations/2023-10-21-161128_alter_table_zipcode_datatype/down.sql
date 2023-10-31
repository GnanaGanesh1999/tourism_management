-- This file should undo anything in `up.sql`
ALTER TABLE Address ALTER COLUMN zip_code TYPE INTEGER USING zip_code::integer;
ALTER TABLE zipcode ALTER COLUMN zip_code TYPE INTEGER USING zip_code::integer;
ALTER TABLE Address ADD CONSTRAINT address_zip_code_fkey FOREIGN KEY (zip_code) REFERENCES Zipcode(zip_code);

