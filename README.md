# Demo Actix-web + Diesel + MySQL

Demo of a simple project using actix-web + SQLX + MariaDB

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Install all used dependencies by using:

``` bash
cargo install sqlx-cli
cargo build
```

Create Database using diesel:

``` bash
sqlx migrate run
```

### Running on DEV environment

``` bash
cargo run
```

### Running using-docker

``` bash
docker compose up
```

### TODO List

- [x] Database connection pooling
- [x] Database migration
- [x] HTTP Auth & permission grants
- [x] Containerization (Docker)
- [x] Image Upload & compression
- [x] Static file serving
- [ ] Consume external API
- [ ] Send Email
- [ ] Cache
