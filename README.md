# FastTQ
 
![alt text](<schema.jpg>)

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