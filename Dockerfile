FROM rust:1.68 as build

# Setup working directory
WORKDIR /usr/src/actix-demo
COPY . .
COPY .env.docker .env

# Build application
RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11

# Application files
COPY --from=build /usr/local/cargo/bin/actix-demo /usr/local/bin/actix-demo
COPY --from=build /usr/src/actix-demo/.env /.env

CMD ["actix-demo"]