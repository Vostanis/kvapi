use dotenv::dotenv;
use serde_json::Value;

// Crypto brokers tend to have pretty modern APIs, as does Binance in the example below.
//
// Some notes about this example:
//      >> the API key is included in the header of the client (and retrieved from our '.env' file);
//      >> 'serde_json::Value' is used as an easy way of exploring the API without having defined the schema, yet.
//
// API Documentation:
//      >> "https://binance-docs.github.io/apidocs/spot/en/#introduction"
//      >> "https://binance-docs.github.io/apidocs/spot/en/#market-data-endpoints"
kvapi::api! {
   name:       Binance
   base:       "https://api.binance.com/api/v3/"
   headers:    {
                   "X-MBX-APIKEY": &key("BINANCE_API"),
                   //              ^ header values must be a referenced String/str
               }
   dict:       {
                   "ping": Value,
                   "exchangeInfo" : Value,
                   #[rename: "BNB_BTC"]    "exchangeInfo?symbol=BNBBTC": Value,
                   #[rename: "sui"]        "ticker/price?symbol=SUIUSDT": Value,
                   #[rename: "sui_btc"]    "ticker/price?symbol=SUIBTC": Value,
                   #[rename: "basket"]     "exchangeInfo?symbols=[\"BTCUSDT\",\"BNBBTC\"]": Value,
                   //                                             ^^       ^^ ^^      ^^
                   //                      speech quotes needed in the URL are handled with the usual backslash
               }
}

pub fn key(var: &str) -> String {
    std::env::var(var).expect("failed to find BINANCE_API in .env")
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let bnc = Binance::new();
    println!("ping! {:#?}", bnc.ping.get().await.expect("ping failed"));

    // // debug info
    bnc.exchange_info.dbg_url();
    bnc.bnb_btc.dbg_url();
    bnc.bnb_btc.dbg_client();

    println!("{:#?}", bnc.bnb_btc.get().await.unwrap());
    println!("{:#?}", bnc.basket.get().await.unwrap());
    println!("{:#?}", bnc.sui.get().await.unwrap());
}
