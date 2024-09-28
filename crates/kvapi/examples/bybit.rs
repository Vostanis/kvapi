mod schema;

use dotenv::{dotenv, var};
use serde_json::Value;

////////////////////////////////////////////////////////////////////////////////////////////////

// API Documentation: "https://bybit-exchange.github.io/docs/v5/intro"
kvapi::api! {
    name:       ByBit
    base:       "https://api.bybit.com/v5/market/"
    headers:    { "apiKey": &var("BYBIT_API").expect("failed to find BYBIT_API env var") }
    dict:       {
                    // let's retrieve some metrics surrounding the SUI/USDC cryptocurrency pair
                    #[rename: "sui_orderbook"] "orderbook?category=spot&symbol=SUIUSDC": Value,
                    #[rename: "sui_open_interest"] "open-interest?category=linear&symbol=SUIUSDT": Value,
                    #[rename: "sui_ratio"] "account-ratio?category=linear&period=1d&symbol=SUIUSDT": Value,
                }
}

////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let bb = ByBit::new();
    // println!("{:#?}", bb.sui_orderbook.get().await?);
    // println!("{:#?}", bb.sui_open_interest.get().await?);
    println!("{:#?}", bb.sui_ratio.get().await?);

    Ok(())
}
