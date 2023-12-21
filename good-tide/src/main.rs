#![feature(thread_id_value)]

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    femme::start();

    let mut app = tide::new();
    app.with(tide::log::LogMiddleware::new());

    app.at("/good").get(|_| async {
        println!(
            "good task, thread id: {}",
            std::thread::current().id().as_u64()
        );
        async_std::task::sleep(std::time::Duration::from_millis(500)).await;
        Ok("good task")
    });
    app.at("/bad").get(|_| async {
        println!(
            "bad task, thread id: {}",
            std::thread::current().id().as_u64()
        );
        async_std::task::sleep(std::time::Duration::from_secs(5)).await;
        for _ in 0..10_000 {
            vec![1; 1_000_000].sort()
        }
        Ok("bad task")
    });
    app.listen("127.0.0.1:3000").await?;
    Ok(())
}
