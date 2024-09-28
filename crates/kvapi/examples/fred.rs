pub mod schema;

use dotenv::dotenv;
use kvapi::{api, Http, HttpMethods};
use schema::fred::{Observations, Releases, Series, Sources};

//////////////////////////////////////////////////////////////////////////////////////////////////////

// The following is an example of the FRED data API.
// This API relies heavily on URL queries (as in: embedding variables into the URL).
//
// API Documentation: `https://fred.stlouisfed.org/docs/api/fred/`
api! {
    name:   Fred
    base:   "https://api.stlouisfed.org/fred"
    query:  format!("api_key={}&file_type=json", key()) // global query, appended to all urls
    dict:   {
                // these queries are key-specific, and are appended before the above (global) query.
                //
                // in this example, category_id=125 would give us Trade Balance data, but the
                // general endpoint would be the same for many datasets.
                //
                // 'rename' and 'query' are useful for these types of APIs.
                //
                // final query: "https://api.stlouisfed.org/fred?category_id=125&api_key={API_KEY}&file_type=json"
                #[query: "?category_id=125&", rename: "trade_balance"]
                "/category/series": Series,

                // setting category_id=100 gives us an array of other datasets, so
                // we can rename it to `other` and explore it.
                #[query: "?category_id=100&", rename: "other"]
                "/category/series": Series,

                // all sources
                #[query="?"] // this '?' is needed for the the global query to remain valid
                "/sources": Sources,

                // U.S. Employment and Training Administration
                #[query="?source_id=50&", rename="employment"]
                "/sources/releases": Releases,

                // 10 yr yield
                #[query="?series_id=DGS10&", rename -> "ten_yr"]
                "/series/observations": Observations,

                // unemployment rate
                #[query: "?series_id=UNRATE&", rename: "unemployment"]
                "/series/observations": Observations,

                // The Federal Reserve releases
                #[query: "?source_id=1&", rename: "the_fed"]
                "/source/releases": Releases,
            }
}

//////////////////////////////////////////////////////////////////////////////////////////////////////

// `.env` file needed with "FRED_API= ..." key
fn key() -> String {
    std::env::var("FRED_API").expect("FRED_API not set")
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    //              `name` variable; i.e., the name of the struct
    //                vv
    let fred: Fred = Fred::new();

    // `.dbg_url()` prints the url that `trade_balance` has created under the hood
    fred.trade_balance.dbg_url(); //
                                  // // explore the `other` dataset
    println!("{:#?}", fred.other.get().await?.series);

    // print all the "sources" section
    for x in fred.sources.get().await?.sources {
        println!(" {:04} | {}", x.id, x.name)
    }

    // print the url, then print all the dates & values
    fred.ten_yr.dbg_url();
    for x in fred.ten_yr.get().await?.inner {
        println!("[{:<10}] {}", x.date, x.value)
    }

    // explore unemployment
    fred.unemployment.dbg_url();
    for x in fred.unemployment.get().await?.inner {
        println!("[{:<10}] {}", x.date, x.value)
    }

    // explore the Fed's releases
    fred.the_fed.dbg_url();
    for x in fred.the_fed.get().await?.releases {
        println!("<< {} >> [{:04}] {}", x.realtime_end, x.id, x.name)
    }

    Ok(())
}
