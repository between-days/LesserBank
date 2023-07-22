
# Bank Resume Project

# Demonstration
Dependencies
- Docker desktop or any kind of docker agent that'll run compose
```docker compose up``` should *just work*™

# Frontend
Nextjs/mantine

## No FE tests yet :eyes:

# Backend
Rustlang with actix-web/diesel

## Database
Postgres with diesel orm
<https://diesel.rs/guides/getting-started>

Balances bigint cents to avoid yucky mappings/manual implementations of to/from table for diesel <https://stackoverflow.com/questions/71815897/rust-correct-use-of-decimal-type-in-diesel>

### Currency
Currency stored as bigint balance_cents in db. Not perfect but good enough for this.

### Migrations
Diesel cli. A little touch and go. Storing balance in cents mig is breaking (changing balance_cents back to balance -> null balance). In practice would need to be setting balance (backwards compat code) or a more complex migration to move values back to balance. Will deal with proper migrations after base functionality done.

## Testing
Using mockall for mocks

Note: Sometimes a `cargo clean` was needed for tests using mockall to compile; issues occuring when compiling new mocks with existing mocks in target

# CI
Currently only for backend.
Pretty basic; clippy for lint, using <https://github.com/Swatinem/rust-cache> for cache. No deployment.

:warning: As of writing, above cache doesn't give option to cache dependencies with test target (<https://github.com/Swatinem/rust-cache/blob/dd05243424bd5c0e585e4b55eb2d7615cdd32f1f/src/workspace.ts#L6>) without caching potentially obsolete packages with ``cache-all-crates``. Might be worth raising a pr/issue for this, would save reinventing the wheel. Using seperate cache for clippy.

## Local Development
Build/run commands like ``cargo run``, ``npm run dev`` etc will work from root. ``Cargo.toml``, ``package.json`` at root map to their respective project sub folders

Steps to develop:
- Spin up demo 
- Stop the container of whatever service you want to work on 
- Run your own version of service as container or with ``cargo run`` or ``npm run dev`` etc