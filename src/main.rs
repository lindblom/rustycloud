use warp::Filter;
use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .unwrap_or("3030".into())
        .parse::<u16>()
        .expect("Port env var not set to a valid u16");
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("hello" / String)
        .map(|name| format!("Hello, {}!", name));

    warp::serve(hello)
        .run(([127, 0, 0, 1], port))
        .await;
}