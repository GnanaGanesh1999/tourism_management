FROM rust:1.51.0

EXPOSE 8000

RUN apt-get update && apt-get install -y libpq-dev

WORKDIR /usr/src/tourist
COPY . .

RUN cargo update

RUN cargo install diesel_cli --no-default-features --features postgres

ENV DATABASE_URL=postgres://root:password@db:5432/touristdb

CMD [ "cargo", "run", "--release"]

