# DBMS Assignment: Tourism Management

This project is for a DBMS assignment on tourism management.

## Folder Structure

- `backend`: This folder contains the Rust code and MySQL migrations using Diesel.

### Pre-requisites
- Docker
- Rust
- Diesel-cli

### Setting up the environment

#### Setting up the database
```bash
docker compose up
```

This command helps to run the Database. It creates a volume when it is running the first time.

#### Installing the diesel-cli
```bash
cargo install diesel_cli --no-default-features --features postgres
diesel migration run
```
This can be done onetime, and can be avoided in the subsequent runs.

#### Setting up the backend
```bash
cargo run --release
```

#### Testing the apis
##### Tour
```bash
curl -X POST -H "Content-Type: application/json" -d '{
    "capacity": "20",
    "cost": "100.0",
    "number_of_days": "7",
    "tour_operator_id": "operator_id",
    "tour_name": "Example Tour",
    "tour_description": "This is an example tour",
    "tour_start_date": "2022-01-01",
    "end_date": "2022-01-07"
}' http://localhost:8000/tour
```

```bash
curl http://localhost:8000/tour
```

##### Tour-operator
```bash
curl -X POST -H "Content-Type: application/json" -d '{
    "mail_id": "example@example.com",
    "tour_operator_password": "example_password",
    "tour_operator_name": "Example Operator"
}' http://localhost:8000/tour-operator
```

```bash
curl http://localhost:8000/tour-operator
```

##### Tourist
```bash
curl -X POST -H "Content-Type: application/json" -d '{"mail_id": "example3@mail.com", "password": "password123", "name": "John Doe", "flat_number": "123", "street": "Main St", "zip_code": "12345", "city": "Anytown", "state": "CA","country":"India"}' http://localhost:8000/tourist
```

```bash
curl http://localhost:8000/tourist
```



