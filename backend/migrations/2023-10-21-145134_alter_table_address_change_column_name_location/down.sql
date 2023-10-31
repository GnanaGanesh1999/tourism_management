-- This file should undo anything in `up.sql`
ALTER TABLE Address DROP CONSTRAINT address_zip_code_fkey;
ALTER TABLE Address RENAME COLUMN zip_code TO zipcode;
ALTER TABLE Address ADD CONSTRAINT address_zipcode_fkey FOREIGN KEY (zipcode) REFERENCES Zipcode(zipcode);