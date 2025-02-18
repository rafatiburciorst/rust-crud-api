# Rust CRUD API

A RESTful API built with Rust, featuring user and post management with authentication.

## Project Structure

```
crud-api/
├── src/
│   ├── config/         # Configuration files (database, etc.)
│   ├── controllers/    # Request handlers
│   ├── entities/       # Data structures and models
│   ├── errors_handler/ # Custom error handling
│   ├── repository/     # Database operations
│   ├── routes/         # API route definitions
│   ├── services/       # Business logic
│   └── main.rs         # Application entry point
├── migrations/         # Database migration files
└── Cargo.toml         # Project dependencies
```

## Dependencies

- actix-web: Web framework
- sqlx: Async PostgreSQL client
- serde: Serialization/deserialization
- jsonwebtoken: JWT authentication
- bcrypt: Password hashing
- validator: Input validation
- chrono: DateTime handling

## Setup

1. Install Rust and Cargo
2. Set up PostgreSQL database
3. Create `.env` file with:
   ```
   DATABASE_URL=postgresql://username:password@localhost/dbname
   ```
4. Run migrations:
   ```bash
   sqlx migrate run
   ```
5. Start the server:
   ```bash
   cargo run
   ```

## API Endpoints

### Users
- GET /users - Get all users
- POST /users - Create new user

### Posts
- Endpoints defined in post_routes.rs

## Authentication

The API uses JWT for authentication with bcrypt for password hashing.
