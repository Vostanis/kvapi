mod schema;

use dotenv::{dotenv, var};
use serde_json::Value;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// API Documentation: https://www.kucoin.com/docs/beginners/introduction
kvapi::api! {
    name:       KuCoin
    base:       "https://api.kucoin.com/api/"
    headers:    {
                    // https://www.kucoin.com/docs/basic-info/connection-method/authentication/creating-a-request
                    "KC-API-KEY": &var("KUCOIN_API").unwrap(),

                    "KC-API-SIGN": &sign(
                        var("KUCOIN_PRIVATE").unwrap(),
                        format!("{}{}", &timestamp(), "GET/api/v3/market/orderbook/level2?symbol=BTC-USDT"),
                    ),

                    "KC-API-TIMESTAMP": &timestamp(),

                    "KC-API-PASSPHRASE": &sign(
                        var("KUCOIN_PRIVATE").unwrap(),
                        var("KUCOIN_PASSPHRASE").unwrap()
                    )

                    "KC-API-KEY-VERSION": "2",

                    "Content-Type": "application/json",
                }

    dict:       {
                    #[rename: "timestamp"]
                    "v1/timestamp" -> Value,

                    // https://www.kucoin.com/docs/rest/spot-trading/market-data/introduction
                    #[rename: "currencies"]
                    "v3/currencies" -> Value,

                    #[rename: "orderbook", query: "?symbol=BTC-USDT"]
                    "v3/market/orderbook/level2" -> Value,
                }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

fn timestamp() -> String {
    chrono::Utc::now().timestamp_millis().to_string()
}

// Signing Documentation: https://www.kucoin.com/docs/basic-info/connection-method/authentication/signing-a-message
fn sign(secret: String, input: String) -> String {
    use base64::prelude::*;
    use hmac::{Hmac, Mac};
    use sha2::Sha256;

    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&secret.as_bytes()).unwrap();
    mac.update(input.as_bytes());
    let result = mac.finalize().into_bytes();

    BASE64_STANDARD.encode(&result)
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    dotenv().ok();

    let kucoin = KuCoin::new();
    // println!("{:#?}", kucoin.currencies.get().await.unwrap());
    println!("{:#?}", kucoin.orderbook.get().await.unwrap());
    // println!("{:#?}", kucoin.timestamp.get().await.unwrap());

    // kucoin.orderbook.dbg_client();
}
