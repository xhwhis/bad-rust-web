#![feature(thread_id_value)]

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route(
            "/good",
            axum::routing::get(|| async {
                println!(
                    "good task, thread id: {}",
                    std::thread::current().id().as_u64()
                );
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
                "good task"
            }),
        )
        .route(
            "/bad",
            axum::routing::get(|| async {
                println!(
                    "bad task, thread id: {}",
                    std::thread::current().id().as_u64()
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                for _ in 0..10_000 {
                    vec![1; 1_000_000].sort()
                }
                "bad task"
            }),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
