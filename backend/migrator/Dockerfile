FROM rust:latest

RUN apt-get update && apt-get install -y libpq-dev

RUN cargo install diesel_cli --no-default-features --features postgres

WORKDIR /app

COPY migrations ./migrations

ENTRYPOINT ["diesel", "migration", "run"]
