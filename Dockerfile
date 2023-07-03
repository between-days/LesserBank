FROM rust:1.70

COPY ./ ./

WORKDIR lesser-bank

RUN cargo build --release

# Using debian since alpine does not find the glibc executables https://pet2cattle.com/2022/11/alpine-not-found
FROM debian:12

# Don't need this yet
# ARG PORT=8080
# ENV HOST=0.0.0.0
# ENV PORT=$PORT
# EXPOSE $PORT

COPY --from=0 lesser-bank/target/release/lesser-bank ./

CMD ["./lesser-bank"]