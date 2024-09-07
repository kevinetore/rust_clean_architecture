<div align="center">
  <h1 align="center">Rust Clean Architecture Api [IN PROGRESS]</h1>

  <p align="center">
    A simple experiment to demonstrate and learn about Rust
  </p>
</div>

## Overview

This project demonstrates how to build a web application using the Rocket framework in Rust with Diesel for database interactions. It’s designed for learning and exploration of the Rust ecosystem.

## Technologies Used

- **Rust**: The programming language.
- **Rocket**: A web framework for building web applications.
- **Diesel**: ORM for Rust.
- **PostgreSQL**: The relational database used.

## Project Structure

- **`core/rocket/`**: Contains the Rocket application code.
- **`core/diesel/`**: Contains the Diesel ORM and schema definitions.
- **`features/`**: Contains the business logic and domain-specific code.

## Running the Application

To run the application, follow these steps:

### Prerequisites

1. **Docker**: Make sure Docker and docker compose are installed on your machine.

### Steps

1. **Build and Run Containers**

   Use Docker Compose to build and start the application and database containers:

   ```sh
   docker-compose up --build
   ```

2. **Access the Application**

Once the containers are running, you can access the Rocket web application at http://localhost:8000.

3. **Running Diesel Migrations**

If you need to manually run Diesel migrations, you can do so by exec-ing into the application container:

```sh
docker-compose exec app bash
diesel migration run
```

## Project Goals

- Exploring the Rust language and ecosystem.


## License

Distributed under the MIT License. See `LICENSE.txt` for more information.