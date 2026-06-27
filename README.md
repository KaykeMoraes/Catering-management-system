# Catering System API

This crate provides the HTTP API server for the Fullstack Catering System.

## Stack

- Rust
- Rocket Framework
- Serde
- SQLx
- Docker
- Postgres

## Environment

The configuration is loaded from environment variables declared in a .env file. Required variables:

- ADDRESS - address Rocket should bind to, e.g. "0.0.0.0" or "127.0.0.1"
- PORT: port Rocket should listen, e.g. 3000
- DATABASE_URL: Postgres connection string, e.g. "postgres://user:pass@host:5432/database"
- ALLOWED_ORIGIN: CORS allowed origin, e.g. "http://origin.com"

See `.env.example` for examples.

## Running locally

You have two options to run this api locally, with docker or cargo.

### With Docker

```bash
git clone https://github.com/KaykeMoraes/Catering-management-system.git
cd Catering-management-system/
docker build -t catering-system-api .
docker run catering-system-api:latest
```

### With Cargo

```bash
git clone https://github.com/KaykeMoraes/Catering-management-system.git
cd Catering-management-system/
cargo run
```

## Next steps

- Add authentication.
- Add role based access.
- Create basic CRUD endpoints.
- Add request and response DTOs.
- Create queries for database operations.
