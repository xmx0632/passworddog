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

    // 创建数据库连接池
    let pool = db::create_pool()
        .await
        .expect("Failed to create database pool");
    
    // 启动HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(actix_web::middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(handlers::login)
                    .service(handlers::create_password)
                    .service(handlers::list_passwords)
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
