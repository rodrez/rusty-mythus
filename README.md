## Rusty Mythus

Rust Backend for Mythus 

- SQLX
- ActixWeb

Start Docker

To start the DB ensure `sqlx-cli` is installed. Then run `scripts/init_db.sh`

After that use `cargo run` `cargo test` or `cargo check`

## Important

We need to run `DATABASE_URL=YOUR-DIGITAL-OCEAN-DB-CONNECTION-STRING sqlx migrate run` every time an update to the database is made.