FROM rust:1.68 AS builder
COPY . .
COPY .env.docker .env
RUN cargo build --release

FROM gcr.io/distroless/cc-debian11
COPY --from=builder ./target/release/actix-demo ./target/release/actix-demo
COPY --from=builder ./.env /target/release/actix-demo/.env
CMD ["/target/release/actix-demo"]
