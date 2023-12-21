#![feature(thread_id_value)]

#[macro_use]
extern crate rocket;

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

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![good, bad])
}
