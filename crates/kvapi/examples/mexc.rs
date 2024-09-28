mod schema;

use serde_json::Value;

// API Documentation: "https://mexcdevelop.github.io/apidocs/spot_v3_en/#introduction"
kvapi::api! {
    name:       Mexc
    base:       "https://api.mexc.com/api/v3/"
    headers:    {
                    "apiKey": &key("MEXC_API"),
                }
    dict:       {
                    "ping": Value,
                }
}

fn key(var: &str) -> String {
    std::env::var(var).expect("failed to find Environment Variable")
}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let mexc = Mexc::new();
    println!("{:#?}", mexc.ping.get().await.unwrap());
}
