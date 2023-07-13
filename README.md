# lesser-bank

bank backend resume project

## Local Deployment

for local testing throw up whole thing with
`docker compose up`
api will be on localhost:8080

## Database

postgres with diesel orm
<https://diesel.rs/guides/getting-started>
balance int for now <https://stackoverflow.com/questions/71815897/rust-correct-use-of-decimal-type-in-diesel>

## Testing
Using mockall for mocks

Note: Sometimes a `cargo clean` is needed for tests using mockall to compile; issues occuring when compiling new mocks with existing target/