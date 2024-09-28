mod schema;

use dotenv::{dotenv, var};
use kvapi::api;
use schema::sec::CompanyTickers;

// API Documentation: "https://www.sec.gov/search-filings/edgar-application-programming-interfaces"
api! {
    name:       Sec
    base:       "https://www.sec.gov/files/"
    headers:    { "User-Agent": &var("USER_AGENT").expect("failed to get User-Agent") }
    dict:       {
                    "company_tickers.json": CompanyTickers,
                }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let sec = Sec::new();
    let tickers = sec.company_tickers.get().await?;
    println!("{:#?}", tickers.entries);
    Ok(())
}
