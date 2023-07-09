FROM rust:1.70 as builder

WORKDIR /api

COPY /api/Cargo.toml /api/Cargo.toml
COPY /api/Cargo.lock /api/Cargo.lock
COPY /api/src /api/src

RUN cargo build --release

# Using debian since alpine does not find the glibc executables https://pet2cattle.com/2022/11/alpine-not-found
FROM debian:12

ENV HOST=0.0.0.0

# need libpq for postgres comms
RUN apt update && apt install -y libpq5

COPY --from=builder api/target/release/lesser-bank-api ./

CMD ["./lesser-bank-api"]