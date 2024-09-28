// pub mod schema;

use serde::Deserialize;

kvapi::api! {
    name:       MyExample
    headers:    {
                    "User-Agent": "hello@me.com"
                }
    dict:       {
                    #[rename: "category", query: env("USER_AGENT")]
                    "/category?id3563.com": MyType
                }
}

fn env(var: &str) -> String {
    std::env::var(var).unwrap()
}

#[derive(Deserialize)]
struct MyType;

#[tokio::main]
async fn main() {
    let api = MyExample::new();
    api.category.get().await.unwrap();
}