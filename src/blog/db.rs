use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteConnectOptions};

#[derive(Clone)]
pub struct StoreDb(pub sqlx::SqlitePool);

pub async fn get_connection_pool(filename: &str) -> Result<StoreDb> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let connection_pool = sqlx::SqlitePool::connect_with(options)
        .await?;
    Ok(StoreDb(connection_pool))
}

pub async fn perform_migrations(db_pool: StoreDb) -> Result<()> {
    sqlx::migrate!("src/blog/migrations")
        .run(&db_pool.0)
        .await?;
    Ok(())
}

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
}

pub async fn list_posts(db_pool: StoreDb) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&db_pool.0)
        .await?;
    Ok(posts)
}

pub async fn get_post(db_pool: StoreDb, id: i32) -> Result<Post> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_one(&db_pool.0)
        .await?;
    Ok(post)
}

pub async fn delete_post(db_pool: StoreDb, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}

pub async fn add_post(db_pool: StoreDb, title: String, author: String) -> Result<()> {
    sqlx::query("INSERT INTO posts (title, author) VALUES (?, ?)")
        .bind(title)
        .bind(author)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}

pub async fn update_post(db_pool: StoreDb, id: i32, title: String, author: String) -> Result<()> {
    sqlx::query("UPDATE posts SET title = ?, author = ? WHERE id = ?")
        .bind(title)
        .bind(author)
        .bind(id)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}