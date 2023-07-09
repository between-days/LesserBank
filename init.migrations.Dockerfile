FROM rust:1.70 as builder

RUN cargo install diesel_cli --no-default-features --features postgres

FROM debian:12

# need libpq for postgres comms
RUN apt update && apt install -y libpq5

COPY ./api/migrations ./migrations
COPY --from=builder /usr/local/cargo/bin/diesel /bin/diesel

CMD diesel migration run