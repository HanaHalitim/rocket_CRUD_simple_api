# Rust Web Application with Rocket

This is a simple Rust-based web application that uses the Rocket framework to manage an in-memory database of records. It supports CRUD operations (Create, Read, Update, Delete) for records through a RESTful API.

## Prerequisites

- [Rust and Cargo](https://www.rust-lang.org/tools/install)

- Rocket requires Rust nightly version (follow Rocket's [getting started guide](https://rocket.rs/v0.5/guide/getting-started/) for setup instructions)

## Setup

### Clone the repository:

    git clone [https url]

### Switch to the nightly toolchain for this project:

    rustup override set nightly

### Build the project:

    cargo build

### Run the application:

    cargo run

## API Endpoints

GET /health: Checks the health of the application.

GET /records: Retrieves all records from the database.

POST /records: Adds a new record to the database.

PUT /records/<id>: Updates an existing record by its ID.

DELETE /records/<id>: Deletes a record by its ID.
