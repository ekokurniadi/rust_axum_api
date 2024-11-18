# Rust Axum API
This project is a RESTful API built with the Rust programming language and the Axum framework. It includes authentication, message broker, CRUD operations for products, categories, and users, and utilizes Docker for deployment.

## Project Structure
```bash
    rust_axum_api/
        Cargo.toml              # Project dependencies and configuration
        Dockerfile              # Docker configuration
        Cargo.lock              # Dependency lock file
        .gitignore              # Ignored files in Git
        docker-compose.yml      # Docker Compose configuration
        .env.example            # Example environment variables
        migrations/             # Database migration files
        src/                    # Source code
            main.rs             # Main application entry point
            middleware/         # Custom middleware
            config/             # Application configuration
            utils/              # Utility functions
            shared/             # Shared modules like error handling
            state/              # State management
            task/               # Task handling modules
            api/                # API modules
                products/       # Products API with models, handlers, services, etc.
                category/       # Category API
                auth/           # Authentication handlers
                users/          # User management API
            routes/             # Route definitions
```
## Key Directories:
- migrations: SQL files for database setup and migration.
- src/api: Contains modules for handling business logic of various entities (products, categories, users, auth).
- src/routes: Route definitions for Axum.
- src/shared: Error handling and shared utilities.

## Prerequisites
- Rust
- Docker
- SQLx CLI for database migrations

## Installation
#### Clone the repository
```bash
git clone https://github.com/ekokurniadi/rust_axum_api.git
cd rust_axum_api
```

#### Copy the example .env file:
```bash
cp .env.example .env
```

#### Update the .env file with your database credentials

## Running Database Migrations
#### Install the SQLx CLI:
```bash
cargo install sqlx-cli
```
#### Run migrations:
```bash
sqlx migrate run
```
## Running the Project
### Using Cargo
#### Run the project in development:
```bash
cd rust_axum_api
cargo run main.rs
```
### Using Docker
#### Build the Docker image:
```bash
docker build -t rust_axum_api .
```
#### Run using docker-compose:
```bash
docker-compose up
```
