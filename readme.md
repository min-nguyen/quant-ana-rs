# Using SQL Data bases in Rust

Add diesel and dotenvy to Cargo.toml:
```toml
[dependencies]
diesel = {version = "1.4.8", features = ["postgres"]}
dotenvy = "0.15.0"
```

## Install Postgresql [https://ubuntu.com/server/docs/install-and-configure-postgresql]
sudo apt install postgresql
sudo apt install libfl-dev
sudo apt install -y postgresql-common
sudo apt-get install postgresql-client
sudo /usr/share/postgresql-common/pgdg/apt.postgresql.org.sh
####### conf is stored in /etc/postgresql/17/main
sudo -u postgres psql template1
ALTER USER postgres with encrypted password 'pw';

## Install Diesel cli [https://diesel.rs/guides/getting-started]
sudo apt-get install libssl-dev
sudo apt-get install libpq-dev
sudo apt-get install postgresql-doc-17
sudo apt install sqlite3
sudo apt install mysql-server
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/diesel-rs/diesel/releases/latest/download/diesel_cli-installer.sh | sh
cargo install diesel_cli --no-default-features --features "postgres"

# Setup Diesel for project [https://diesel.rs/guides/getting-started]
echo DATABASE_URL=postgres://postgres:pw@localhost/my_db > .env
diesel setup --database-url='postgres://postgres:pw@localhost/my_db'
diesel migration generate create_trades ### create a migration to initialize a database schema

# Write SQL for migrations

Write in migrations/down.sql and migrations.up.sql.
```sql
-- Up migration
CREATE TABLE trades (
    id SERIAL PRIMARY KEY,
    symbol VARCHAR NOT NULL,
    price DECIMAL(10, 2) NOT NULL,
    volume INT NOT NULL,
    trade_time TIMESTAMP NOT NULL
)
```
```sql
-- Down migration
DROP TABLE trades
```

Run the migration to apply changes:
```bash
diesel migration run`
```