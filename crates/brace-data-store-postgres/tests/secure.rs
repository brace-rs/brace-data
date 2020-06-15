#![cfg(all(feature = "tls", feature = "snakeoil"))]

use std::str::FromStr;

use brace_data_store_postgres::{Config, Error, Postgres};

#[tokio::test]
async fn test_postgres_connect_secure() -> Result<(), Error> {
    let config = Config::from_str("postgresql://postgres:postgres@localhost:5432")?;
    let postgres = Postgres::snakeoil(config).await?;
    let conn = postgres.connect().await?;
    let row = conn.query_one("SELECT 52", &[]).await?;
    let res = row.get::<usize, i32>(0);

    assert_eq!(res, 52);

    Ok(())
}
