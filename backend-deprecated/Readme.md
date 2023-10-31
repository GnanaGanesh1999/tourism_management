This folder contains the Rust code and MySQL migrations using Diesel.


## Steps to Run Backend

1. Ensure MySQL server is running on localhost port 3306. Check the `backend/.env` file for configuration details.
2. Install Rust.
3. Navigate to the backend directory with `cd backend`.
4. Install Diesel CLI with MySQL features using the command `cargo install diesel_cli --no-default-features --features mysql`
5. Run Diesel migrations with the command `diesel migration run`
