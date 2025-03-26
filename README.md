# use-rs
### Simple Rust demo application, to show how to expose a REST API with Axum for CRUD ops on Postgres connecting via Diesel.

Key improvements and explanations
- Connection Pooling (r2d2): Uses r2d2 for efficient database connection management, improving performance and avoiding connection exhaustion.
- Error Handling: Implements proper error handling, returning appropriate HTTP status codes (e.g., 404 for not found, 500 for internal server errors).
- Diesel Integration: Uses Diesel for type-safe database interactions, preventing SQL injection vulnerabilities and improving code readability.
- Axum REST API: Exposes a RESTful API using Axum, a modern and efficient Rust web framework.
- Serde for JSON Handling: Uses Serde for easy serialization and deserialization of JSON data.
- Clearer Data Structures: Defines User, NewUser, and UpdateUser structs for better data representation.
- Environment Variables: Uses dotenvy to load database credentials from a .env file, keeping sensitive information out of the code.
- Updated Axum and Diesel versions: Uses the latest Axum 0.7 and Diesel 2.1 versions.
- Clearer error messaging: Uses map_err to provide more informative error messages.
- Updated table definition: Uses table! macro for cleaner diesel table definition.
- Tokio async runtime: Uses tokio for async operations.
