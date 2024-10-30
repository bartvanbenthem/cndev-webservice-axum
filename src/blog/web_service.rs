use axum::{http::StatusCode, Extension, Json};
use super::db::{self, Post, StoreDb};

pub async fn all_posts(
    Extension(db_pool): Extension<StoreDb>
) -> Result<Json<Vec<Post>>, StatusCode> {
    let posts = db::list_posts(db_pool).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(posts))
}

pub async fn get_post(
    Extension(db_pool): Extension<StoreDb>,
    path: axum::extract::Path<i32>
) -> Result<Json<Post>, StatusCode> {
    let post = db::get_post(db_pool, path.0).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(post))
}

pub async fn delete_post(
    Extension(db_pool): Extension<StoreDb>,
    path: axum::extract::Path<i32>
) -> Result<StatusCode, StatusCode> {
    db::delete_post(db_pool, path.0).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn add_post(
    Extension(db_pool): Extension<StoreDb>,
    Json(post): Json<Post>
) -> Result<StatusCode, StatusCode> {
    db::add_post(db_pool, post.title, post.author).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}

pub async fn update_post(
    Extension(db_pool): Extension<StoreDb>,
    path: axum::extract::Path<i32>,
    Json(post): Json<Post>
) -> Result<StatusCode, StatusCode> {
    db::update_post(db_pool, path.0, post.title, post.author).await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(StatusCode::OK)
}