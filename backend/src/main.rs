use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use log::info;

mod models;
mod handlers;
mod db;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // 加载环境变量
    dotenv().ok();
    
    // 初始化日志
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    info!("Starting PasswordDog server...");
    
    // 启动HTTP服务器
    HttpServer::new(|| {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            // TODO: 添加路由
            .service(
                web::scope("/api")
                    // 后续添加具体的路由
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
