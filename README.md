# TacoQ(ueue)

## SQLX Type Checking

To be able to type check sqlx queries, you need to do the following:

1. Start up the Postgres database

```bash
docker compose up postgres
```

2. Set the DATABASE_URL environment variable to the connection string for the postgres database:

```bash
DATABASE_URL="postgresql://user:password@localhost:5432/fasttq"
```

3. Run the following command to type check the sqlx queries:

```bash
cargo sqlx prepare
```

## Testing

To run the tests, your database and RabbitMQ/Redis instances must be running.

### Rust Tests:

```bash
cargo test
```

### Python Client Tests:

The python client requires UV to be installed. To install UV, run the following command:

```bash
# On macOS and Linux.
curl -LsSf https://astral.sh/uv/install.sh | sh
```

```powershell
# On Windows.
powershell -ExecutionPolicy ByPass -c "irm https://astral.sh/uv/install.ps1 | iex"
```

To run the tests, navigate to the python client directory and run the following command:

```bash
cd clients/client_sdks/python
uv run pytest
```
