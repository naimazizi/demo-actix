# Demo Actix-web + Diesel + MySQL

Demo of a simple project using actix-web + Disel + MySQL

## Getting Started

These instructions will get you a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

Install all used dependencies by using:

```
cargo build
cargo install diesel_cli --no-default-features --features mysql
```

Create Database using diesel:

```
diesel setup
```

### Running on DEV environment

```
cargo run
```