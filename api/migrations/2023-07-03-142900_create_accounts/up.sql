CREATE TABLE accounts (
  id SERIAL PRIMARY KEY,
  customer_id integer not null,
  balance integer not null
)