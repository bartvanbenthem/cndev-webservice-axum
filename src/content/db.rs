use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, sqlite::SqliteConnectOptions};

#[derive(Clone)]
pub struct ContentDb(pub sqlx::SqlitePool);

pub async fn get_connection_pool(filename: &str) -> Result<ContentDb> {
    let options = SqliteConnectOptions::new()
        .filename(filename)
        .create_if_missing(true);

    let connection_pool = sqlx::SqlitePool::connect_with(options).await?;
    Ok(ContentDb(connection_pool))
}

pub async fn perform_migrations(db_pool: ContentDb) -> Result<()> {
    sqlx::migrate!("src/content/migrations")
        .run(&db_pool.0)
        .await?;
    Ok(())
}

// CONTENT BLOG POSTS ///

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub author: String,
    pub img: String,
    pub content: String,
}

pub async fn list_posts(db_pool: ContentDb) -> Result<Vec<Post>> {
    let posts = sqlx::query_as::<_, Post>("SELECT * FROM posts")
        .fetch_all(&db_pool.0)
        .await?;
    Ok(posts)
}

pub async fn get_post(db_pool: ContentDb, id: i32) -> Result<Post> {
    let post = sqlx::query_as::<_, Post>("SELECT * FROM posts WHERE id = ?")
        .bind(id)
        .fetch_one(&db_pool.0)
        .await?;
    Ok(post)
}

pub async fn delete_post(db_pool: ContentDb, id: i32) -> Result<()> {
    sqlx::query("DELETE FROM posts WHERE id = ?")
        .bind(id)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}

pub async fn add_post(
    db_pool: ContentDb,
    title: String,
    author: String,
    img: String,
    content: String,
) -> Result<()> {
    sqlx::query("INSERT INTO posts (title, author, img, content) VALUES (?, ?, ?, ?)")
        .bind(title)
        .bind(author)
        .bind(img)
        .bind(content)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}

pub async fn update_post(
    db_pool: ContentDb,
    id: i32,
    title: String,
    author: String,
    img: String,
    content: String,
) -> Result<()> {
    sqlx::query("UPDATE posts SET title = ?, author = ?, img = ?, content = ? WHERE id = ?")
        .bind(title)
        .bind(author)
        .bind(img)
        .bind(content)
        .bind(id)
        .execute(&db_pool.0)
        .await?;
    Ok(())
}

// CONTENT ABOUT ///

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct About {
    pub id: i32,
    pub content: String,
}

pub async fn get_about(db_pool: ContentDb) -> Result<About> {
    let about_content = sqlx::query_as::<_, About>("SELECT * FROM about ORDER BY id DESC LIMIT 1")
        .fetch_one(&db_pool.0)
        .await?;
    Ok(about_content)
}

// CONTENT SERVICES ///
#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Service {
    pub id: i32,
    pub title: String,
    pub category: String,
    pub summary: String,
    pub img: String,
    pub content: String,
}

pub async fn list_services(db_pool: ContentDb) -> Result<Vec<Service>> {
    let services = sqlx::query_as::<_, Service>("SELECT * FROM services")
        .fetch_all(&db_pool.0)
        .await?;
    Ok(services)
}

pub async fn get_service(db_pool: ContentDb, id: i32) -> Result<Service> {
    let service = sqlx::query_as::<_, Service>("SELECT * FROM services WHERE id = ?")
        .bind(id)
        .fetch_one(&db_pool.0)
        .await?;
    Ok(service)
}
