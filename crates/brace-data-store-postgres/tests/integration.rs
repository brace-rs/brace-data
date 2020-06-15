use std::str::FromStr;

use tokio_postgres::{Config, Error};

use brace_data_store_postgres::Postgres;

#[tokio::test]
async fn test_postgres_connect() -> Result<(), Error> {
    let config = Config::from_str("postgresql://postgres:postgres@localhost:5432")?;
    let postgres = Postgres::new(config);
    let conn = postgres.connect().await?;
    let row = conn.query_one("SELECT 42", &[]).await?;
    let res = row.get::<usize, i32>(0);

    assert_eq!(res, 42);

    Ok(())
}
