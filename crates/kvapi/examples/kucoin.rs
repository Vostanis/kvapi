mod schema;

use dotenv::{dotenv, var};
use serde_json::Value;

// API Documentation: "https://www.kucoin.com/docs/rest/margin-trading/market-data"
kvapi::api! {
    name:       KuCoin
    base:       "https://api.kucoin.com/api/v3/"
    headers:    {
                    // KC-API-KEY The API key as a string.
                    "KC-API-KEY": &var("KUCOIN_API")
                        .expect("failed to find KUCOIN_API env. var."),

                    // KC-API-SIGN The base64-encoded signature (see Signing a Message).
                    "KC-API-SIGN": "mycodehere98127339812731298",

                    // KC-API-TIMESTAMP A timestamp for your request.
                    "KC-API-TIMESTAMP": &timestamp(),

                    // KC-API-PASSPHRASE The passphrase you specified when creating the API key.
                    "KC-API-PASSPHRASE": &var("KUCOIN_PASSPHRASE")
                        .expect("failed to find KUCOIN_PASSPHRASE env. var."),

                    // KC-API-KEY-VERSION You can check the version of API key on the page of API Management
                    "KC-API-KEY-VERSION": "2",
                }

    dict:       {
                    "currencies" -> Value,
                }
}

fn timestamp() -> String {
    String::from("string_here")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let kucoin = KuCoin::new();
    println!("{:#?}", kucoin.currencies.get().await.unwrap());
}
