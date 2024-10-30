use super::db;
use axum::{
    extract::Request,
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Extension,
};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub struct ValidUser(pub i32);

pub async fn require_token(
    Extension(db_pool): Extension<db::AuthDb>,
    headers: HeaderMap,
    mut req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth_header) = headers.get("Token") {
        let token = auth_header
            .to_str()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid header".to_string()))?;

        if let Some(user_id) = db::get_user_id_from_token(db_pool, token)
            .await
            .map_err(|_| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database error".to_string(),
                )
            })?
        {
            req.extensions_mut().insert(ValidUser(user_id));
            return Ok(next.run(req).await);
        }
    }

    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}

/// Used to target requests to the auth service. In a large service,
/// you'd use a service locator system or multiple environment
/// variables. We're using this to keep it simple.
#[derive(Clone)]
pub struct ListenPort(pub String);

pub async fn require_remote_token(
    Extension(listen_port): Extension<ListenPort>,
    headers: HeaderMap,
    req: Request,
    next: Next,
) -> Result<impl IntoResponse, (StatusCode, String)> {
    if let Some(auth_header) = headers.get("Token") {
        let token = auth_header
            .to_str()
            .map_err(|_| (StatusCode::UNAUTHORIZED, "invalid header".to_string()))?;

        let url = format!(
            "http://localhost:{}/api/auth/is_token_valid/{token}",
            listen_port.0
        );
        let _body = reqwest::get(&url)
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid token request".to_string(),
                )
            })?
            .text()
            .await
            .map_err(|_| {
                (
                    StatusCode::UNAUTHORIZED,
                    "Invalid token request".to_string(),
                )
            })?;
        tracing::info!("Validated Token");

        return Ok(next.run(req).await);
    }

    Err((StatusCode::UNAUTHORIZED, "invalid header".to_string()))
}
