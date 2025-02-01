use actix_web::{get, post, web, HttpResponse, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct CreatePasswordRequest {
    title: String,
    username: String,
    password: String,
    website: Option<String>,
    notes: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PasswordResponse {
    id: i64,
    title: String,
    username: String,
    website: Option<String>,
    notes: Option<String>,
}

#[post("/passwords")]
pub async fn create_password(
    _password_data: web::Json<CreatePasswordRequest>,
) -> Result<HttpResponse> {
    // TODO: 实现创建密码逻辑
    Ok(HttpResponse::Ok().finish())
}

#[get("/passwords")]
pub async fn list_passwords() -> Result<HttpResponse> {
    // TODO: 实现获取密码列表逻辑
    Ok(HttpResponse::Ok().json(Vec::<PasswordResponse>::new()))
}
