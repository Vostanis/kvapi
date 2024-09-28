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