mod schema;

use base64::prelude::{Engine, BASE64_STANDARD};
use dotenv::{dotenv, var};
use hmac::{Hmac, Mac};
use serde_json::Value;
use sha2::Sha256;

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// API Documentation: https://www.kucoin.com/docs/beginners/introduction
kvapi::api! {
    name: KuCoin
    base: "https://api.kucoin.com/api/"
    head: {
              // https://www.kucoin.com/docs/basic-info/connection-method/authentication/creating-a-request
              "KC-API-KEY": &var("KUCOIN_API")?,
              "KC-API-KEY-VERSION": "2",

              // the next 3 headers require the timestamp to be the same, so they're performed at
              // the query level, for reusage
              #[query]
              "KC-API-TIMESTAMP": &timestamp(),

              #[query]
              "KC-API-PASSPHRASE": &encrypt(
                  var("KUCOIN_PRIVATE").unwrap(),
                  var("KUCOIN_PASSPHRASE").unwrap(),
              ),

              #[query]
              "KC-API-SIGN": &sign(
                  &self.url, // one can access the url & client of `self`
                             // this could be risky if not used properly
                  var("KUCOIN_PRIVATE").unwrap(),
                  timestamp(),
              ),
          }

    dict: {
              #[rename: "timestamp"]
              "v1/timestamp" -> Value,

              // https://www.kucoin.com/docs/rest/spot-trading/market-data/introduction
              #[rename: "currencies"]
              "v3/currencies" -> Value,

              #[rename: "orderbook", query: "?symbol=BTC-USDT"]
              "v3/market/orderbook/level2" -> Value,

              #[rename: "candles"]
              "v1/market/candles?type=1day&symbol=BTC-USDT&startAt=1566703297&endAt=1566789757" -> Value,
          }
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

// Signing Documentation: https://www.kucoin.com/docs/basic-info/connection-method/authentication/signing-a-message
fn encrypt(secret: String, input: String) -> String {
    type HmacSha256 = Hmac<Sha256>;
    let mut mac = HmacSha256::new_from_slice(&secret.as_bytes()).unwrap();
    mac.update(input.as_bytes());
    let result = mac.finalize().into_bytes();
    let b64 = BASE64_STANDARD.encode(&result);
    b64
}

fn sign(url: &String, secret: String, timestamp: String) -> String {
    let url = url.replace("https://api.kucoin.com", "");
    let input = format!("{}{}{}", timestamp, "GET", url);
    encrypt(secret, input)
}

fn timestamp() -> String {
    chrono::Utc::now().timestamp_millis().to_string()
}

///////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    dotenv().ok();

    let kucoin = KuCoin::new();
    println!("{:#?}", kucoin.currencies.get().await.unwrap());
    println!("{:?}", kucoin.candles.get().await.unwrap());
}
