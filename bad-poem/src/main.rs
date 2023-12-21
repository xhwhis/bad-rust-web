#![feature(thread_id_value)]

use poem::{get, handler, listener::TcpListener, Route, Server};

#[handler]
async fn good() -> &'static str {
    println!(
        "good task, thread id: {}",
        std::thread::current().id().as_u64()
    );
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    "good task"
}

#[handler]
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

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let app = Route::new().at("/good", get(good)).at("/bad", get(bad));
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
