# Axum and Tokio REST API Demo

This project is a simple demonstration of how to create a REST API using
[Axum](https://github.com/tokio-rs/axum) and [Tokio](https://tokio.rs/).

The Youtube related video is https://youtu.be/cgej2h-GeoQ

It was created for a YouTube tutorial aimed at people who have never worked
with Axum before. The example implements only the `/users` endpoint to
introduce the basic concepts of routing, JSON serialization, request handling,
and HTTP status codes.

## Important note

This project is for educational purposes only. It does not represent a real
production environment and should not be used as a production-ready API.

## Running the project

Make sure you have Rust installed, then run:

```bash
cargo run
```

The API will be available at `http://localhost:8080`.

## Endpoint

### `POST /users`

Creates a user from a JSON request body.

Example request:

```bash
curl -X POST http://localhost:8080/users \
  -H "Content-Type: application/json" \
  -d '{"nome":"Fernando"}'
```

Successful responses return HTTP `201 Created` with the created user. If the
user name is `teste`, the API returns HTTP `400 Bad Request`.
