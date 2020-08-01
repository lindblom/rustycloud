use warp::http::header::HeaderValue;
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
        .map(|name| format!("Hello, {}!", name))
        .with(warp::reply::with::header("Cache-Control", HeaderValue::from_static("no-cache")));


    println!("Starting server on port {}", port);
    warp::serve(hello)
        .run(([0, 0, 0, 0], port))
        .await;
    println!("Server has been terminate");
}