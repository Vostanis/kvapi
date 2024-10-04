mod schema;

use dotenv::{dotenv, var};
use serde_json::Value;

///////////////////////////////////////////////////////////////////////////////////////////

// API Documentation: "https://mexcdevelop.github.io/apidocs/spot_v3_en/#introduction"
kvapi::api! {
    name:       Mexc
    base:       "https://api.mexc.com/api/v3/"
    headers:    {
                    "apiKey": &var("MEXC_API").expect("failed to find MEXC_API"),
                }
    dict:       {
                    "ping": Value,

                    #[query: "?symbols=BTCUSDT,ETHUSDT,SOLUSDT,KASUSDT,ALEPHUSDT,SUIUSDT"]
                    "exchangeInfo": Value,

                    #[rename="kaspa_klines", query: "?symbol=KASUSDT&interval=1d"]
                    "klines": Value,

                    #[rename: "btc_orderbook", query: "?symbol=BTCUSDT"]
                    "depth": Value,
                }
}

///////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() {
    dotenv().ok();

    // each endpoint will have its own client, and all are initialised at the point of `new()`
    let mexc = Mexc::new();
    println!("{:#?}", mexc.ping.get().await.unwrap());
    println!("{:#?}", mexc.exchange_info.get().await.unwrap());
    println!("{:#?}", mexc.kaspa_klines.get().await.unwrap());
    println!("{:#?}", mexc.btc_orderbook.get().await.unwrap());
}
