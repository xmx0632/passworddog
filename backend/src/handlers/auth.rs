use actix_web::{post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String,
}

#[post("/login")]
pub async fn login(
    _login_data: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    // TODO: 实现登录逻辑
    Ok(HttpResponse::Ok().json(LoginResponse {
        token: "dummy_token".to_string(),
    }))
}
