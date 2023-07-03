FROM rust:1.70 as builder

COPY ./ ./

WORKDIR api

RUN cargo build --release

# Using debian since alpine does not find the glibc executables https://pet2cattle.com/2022/11/alpine-not-found
FROM debian:12

ENV HOST=0.0.0.0
# Don't need this yet
# ARG PORT=8080
# ENV PORT=$PORT
# EXPOSE $PORT

COPY --from=builder api/target/release/lesser-bank-api ./

CMD ["./lesser-bank-api"]