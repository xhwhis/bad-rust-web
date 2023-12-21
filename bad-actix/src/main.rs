#![feature(thread_id_value)]

use actix_web::{get, App, HttpServer};

#[get("/good")]
async fn good() -> &'static str {
    println!(
        "good task, thread id: {}",
        std::thread::current().id().as_u64()
    );
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    "good task"
}

#[get("/bad")]
async fn bad() -> &'static str {
    println!(
        "bad task, thread id: {}",
        std::thread::current().id().as_u64()
    );
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    for _ in 0..10_000 {
        vec![1; 1_000_000].sort()
    }
    "bad task"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(good).service(bad))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
