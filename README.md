# Intro
`kvapi` is all about shortcuts for working with APIs in **rust**;
the procedural macro `api!` provides a framework for working with RESTful APIs.

## Example

Below is an example of a single endpoint from the **SEC (Security Exchange Commission)** API, known as **EDGAR**.

It simply returns the endpoint: "https://www.sec.gov/files/company_tickers.json", paired with a Deserializable type, `CompanyTickers`.

It also uses the Environment Variable "USER_AGENT", loading it with the [dotenv] crate from a .env file.

```rust

// main.rs
use dotenv::{dotenv, var};
use kvapi::api;
use schema::sec::CompanyTickers;

// API Documentation: "https://www.sec.gov/search-filings/edgar-application-programming-interfaces"
api! {
    name:       Sec
    base:       "https://www.sec.gov/files/"
    //          we could ignore `base`, but it's used here for showcasing purposes
    headers:    { "User-Agent": &var("USER_AGENT").expect("failed to get User-Agent") }
    //                              ^^^
    //          we can use functions in the definitions to pull Environment Variables
    dict:       {
                    "company_tickers.json": CompanyTickers,
                }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let sec = Sec::new();
    let tickers = sec.company_tickers.get().await?;
    //                      ^^^
    //          the dict entry 
    println!("{:#?}", tickers.entries);
    Ok(())
}

```

This draws a Deserializable type from the below file, where the [serde] crate is used in defining our schema.

```rust

// schema/sec.rs
use std::collections::BTreeMap;

use serde::Deserialize;

#[derive(Debug)]
pub struct CompanyTickers {
    pub entries: Vec<Entry>,
}

#[derive(Debug, Deserialize)]
pub struct Entry {
    pub cik_str: u32,
    pub ticker: String,
    pub title: String,
}

impl<'de> Deserialize<'de> for CompanyTickers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let btree: BTreeMap<u32, Entry> = Deserialize::deserialize(deserializer)?;
        let entries: Vec<Entry> = btree.into_iter().map(|(_, entry)| entry).collect();
        Ok(CompanyTickers { entries })
    }
}

```

[dotenv]: https://docs.rs/dotenv/latest/dotenv/
[serde]: https://docs.rs/serde/latest/serde/
