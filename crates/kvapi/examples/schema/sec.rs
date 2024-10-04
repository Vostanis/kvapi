use serde::{
    de::{MapAccess, Visitor},
    Deserialize,
};

/// The `Tickers` struct is used to deserialize the `company_tickers.json` file.
///
/// It has been heavily redefined as an example of how one could optimise the deserialization
/// process, using `serde` in the backend, and still maintaining a clean API on the front-end.
#[derive(Debug)]
pub struct Tickers(pub Vec<Ticker>);

#[derive(Debug, Deserialize)]
pub struct Ticker {
    #[serde(rename = "cik_str")]
    pub id: u32,
    pub ticker: String,
    pub title: String,
}

pub struct TickerVisitor;

impl<'de> Visitor<'de> for TickerVisitor {
    type Value = Tickers;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("tickers in the form of a map")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut tickers: Vec<Ticker> = Vec::new();

        // each entry is in the form of:
        // `0: { "cik_str": 320193, "ticker": "AAPL", "title": "Apple Inc." },`
        while let Some((_, ticker)) = map.next_entry::<u16, Ticker>()? {
            tickers.push(ticker);
        }

        Ok(Tickers(tickers))
    }
}

impl<'de> Deserialize<'de> for Tickers {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // we want a vector returned, but the deserialize will expect a map, given
        // how the API has been designed
        deserializer.deserialize_map(TickerVisitor)
    }
}
