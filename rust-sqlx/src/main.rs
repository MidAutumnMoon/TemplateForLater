use std::str::FromStr;

use anyhow::Context;
use sqlx::SqlitePool;
use sqlx::sqlite::SqliteConnectOptions;

#[derive(Debug)]
struct SayHello {
    id: i64,
    message: String,
    luck_number: i64,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let opts =
        SqliteConnectOptions::from_str(&std::env::var("DATABASE_URL")?)
            .context("Failed to parse DATABASE_URL")?
            .journal_mode(sqlx::sqlite::SqliteJournalMode::Wal);
    let pool = SqlitePool::connect_with(opts).await?;

    let rn = rand::random::<i64>();

    sqlx::query!(
        "insert into say_hello (message, luck_number) values (?, ?)",
        "hello",
        rn
    )
    .execute(&pool)
    .await?;

    let rows = sqlx::query_as!(SayHello, "select * from say_hello")
        .fetch_all(&pool);

    dbg!(rows.await?);

    sqlx::query!(
        r#"insert into posts (author, content) values ("hello", "world")"#
    )
    .execute(&pool)
    .await?;

    /// ?w
    #[derive(Debug)]
    struct Post {
        author: Option<String>,
        content: Option<String>,
    }

    let rows = sqlx::query_as!(
        Post,
        r#"
            select
                author as "author: String",
                content as "content: String"
            from posts
            where
                content match ?
        "#,
        "world"
    )
    .fetch_all(&pool)
    .await?;

    dbg!(&rows);

    Ok(())
}
