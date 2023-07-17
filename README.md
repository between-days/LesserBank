# lesser-bank

Bank backend resume project

## Local Deployment

For local testing throw up whole thing with
`docker compose up`
api will be on localhost:8080

## Currency
Currency stored as bigint balance_cents in db. Not perfect but good enough for this.

## Database

Postgres with diesel orm
<https://diesel.rs/guides/getting-started>
balance int for now <https://stackoverflow.com/questions/71815897/rust-correct-use-of-decimal-type-in-diesel>

#### Migrations
Diesel cli. A little touch and go. Storing balance in cents mig is breaking (changing balance_cents back to balance -> null balance). In practice would need to need to be setting balance (backwards compat code) or a more complex migration to move values back to balance. 

## Testing
Using mockall for mocks

Note: Sometimes a `cargo clean` is needed for tests using mockall to compile; issues occuring when compiling new mocks with existing target/