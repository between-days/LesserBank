FROM rust:1.70

ARG PORT=8080

COPY ./ ./

WORKDIR lesser-bank

RUN cargo build --release

ENV HOST=0.0.0.0
ENV PORT=$PORT

EXPOSE $PORT

CMD ["./target/release/lesser-bank"]