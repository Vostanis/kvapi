mod schema;

use dotenv::{dotenv, var};
use kvapi::api;
use schema::sec::Tickers;

// A very simple example of the SEC's EDGAR API, in which we simply return the list of company tickers.
// The `schema::sec` module is used to intricately define exact `serde` Deserialization for the JSON.
// (It might shed some light on how to optimise `serde`, as well.)
//
// API Documentation: https://www.sec.gov/search-filings/edgar-application-programming-interfaces
api! {
    name: Sec
    base: "https://www.sec.gov/files/"
    head: {
        "User-Agent": &var("USER_AGENT")?
        #[query] "custom": &my_func()
    }
    dict: { "company_tickers.json" -> Tickers }
}

#[allow(dead_code)]
fn my_func() -> String {
    "kimonvostanis@gmail.com".to_string()
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let sec = Sec::new();
    let tickers = sec.company_tickers.get().await?;
    println!("{:#?}", tickers.0);

    Ok(())
}
