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

## CI
Pretty basic; clippy for lint, using <https://github.com/Swatinem/rust-cache> for cache. No deployment.

:warning: As of writing, above cache doesn't give option to cache dependencies with test target (<https://github.com/Swatinem/rust-cache/blob/dd05243424bd5c0e585e4b55eb2d7615cdd32f1f/src/workspace.ts#L6>) without caching potentially obsolete packages with ```cache-all-crates```. Might be worth raising a pr/issue for this, would save reinventing the wheel. Using seperate cache for clippy.