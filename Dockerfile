FROM rustlang/rust:nightly AS builder

WORKDIR /usr/src/app

# Copy project files
COPY Cargo.toml Cargo.lock ./
COPY core/diesel/ core/diesel/
COPY core/rocket/ core/rocket/
COPY features/ features/

RUN cargo build --release

FROM rustlang/rust:nightly

WORKDIR /usr/src/app

COPY --from=builder /usr/src/app/target/release/ /usr/src/app/

# Install necessary system dependencies
RUN apt-get update && apt-get install -y libpq-dev

# Install Diesel CLI to run migrations
RUN cargo install diesel_cli --no-default-features --features postgres

# Copy diesel.toml, schema, and migration files
COPY core/diesel/diesel.toml ./
COPY core/diesel/tables/schema.rs core/diesel/tables/schema.rs
COPY core/diesel/migrations/ core/diesel/migrations/

ENV ROCKET_ADDRESS=0.0.0.0
EXPOSE 8000