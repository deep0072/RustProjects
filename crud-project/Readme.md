# CRUD App

## Introduction

This project is a CRUD (Create, Read, Update, Delete) application built with Rust, PostgreSQL, Docker. It demonstrates how to set up a simple web application that interacts with a database, all within Docker containers for easy deployment and scaling.

## Stack

- **Rust**: The programming language used to build the application.
- **PostgreSQL**: The database system used to store and manage the application's data.
- **Docker**: A platform used to develop, ship, and run applications inside containers.


## How to Run

### Running Locally

To run the application locally, you need to have PostgreSQL running on your machine. Then, execute the following command in your terminal:

```bash
cargo run
```

### Running with Docker Compose

To set up everything, including the database, using Docker Compose, follow these steps:

1. Build the Docker image for the application:

```bash
docker build . -t crud:latest
```

2. Navigate to the `docker` directory and start the application:

```bash
cd docker && docker compose up
```

This command will build the Docker image for your application, set up the PostgreSQL database, and start both services.




