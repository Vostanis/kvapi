use serde::Deserialize;

// {
//     "timestamp": 3987128312,
//     "categories": [
//         {
//             "id": 125,
//             "name": "Trade Balance",
//             "parent_id": 13
//         }
//     ]
//     "error": []
// }
#[derive(Debug, Deserialize)]
pub struct Categories {
    pub categories: Vec<Category>,
}

#[derive(Debug, Deserialize)]
pub struct Category {
    pub id: u32,
    pub name: String,
    pub parent_id: u32,
}

// {
//     "realtime_start": "2017-08-01",
//     "realtime_end": "2017-08-01",
//     "order_by": "series_id",
//     "sort_order": "asc",
//     "count": 45,
//     "offset": 0,
//     "limit": 1000,
//     "seriess": [
//       {
//         "id": "BOPBCA",
//         "realtime_start": "2017-08-01",
//         "realtime_end": "2017-08-01",
//         "title": "Balance on Current Account (DISCONTINUED)",
//         "observation_start": "1960-01-01",
//         "observation_end": "2014-01-01",
//         "frequency": "Quarterly",
//         "frequency_short": "Q",
//         "units": "Billions of Dollars",
//         "units_short": "Bil. of $",
//         "seasonal_adjustment": "Seasonally Adjusted",
//         "seasonal_adjustment_short": "SA",
//         "last_updated": "2014-06-18 08:41:28-05",
//         "popularity": 32,
//         "group_popularity": 34,
//         "notes": "This series has been discontinued as a result of the comprehensive restructuring of the international economic accounts (http:\/\/www.bea.gov\/international\/modern.htm). For a crosswalk of the old and new series in FRED see: http:\/\/research.stlouisfed.org\/CompRevisionReleaseID49.xlsx."
//       },
#[derive(Deserialize, Debug)]
pub struct SeriesEntry {
    pub id: String,
    pub title: String,
    pub realtime_start: String,
    pub frequency: String,
    pub popularity: u32,
}

#[derive(Deserialize, Debug)]
pub struct Series {
    #[serde(rename = "seriess")] // there's an extra 's' in the API (for some reason)
    pub series: Vec<SeriesEntry>,
}

// {
//     "realtime_start": "2013-08-14",
//     "realtime_end": "2013-08-14",
//     "order_by": "source_id",
//     "sort_order": "asc",
//     "count": 58,
//     "offset": 0,
//     "limit": 1000,
//     "sources": [
//         {
//             "id": 1,
//             "realtime_start": "2013-08-14",
//             "realtime_end": "2013-08-14",
//             "name": "Board of Governors of the Federal Reserve System",
//             "link": "http://www.federalreserve.gov/"
#[derive(Deserialize, Debug)]
pub struct Sources {
    pub sources: Vec<Source>,
}

#[derive(Deserialize, Debug)]
pub struct Source {
    pub id: u32,
    pub realtime_start: String,
    pub realtime_end: String,
    pub name: String,
}

// {
//     "realtime_start": "2013-08-14",
//     "realtime_end": "2013-08-14",
//     "order_by": "release_id",
//     "sort_order": "asc",
//     "count": 26,
//     "offset": 0,
//     "limit": 1000,
//     "releases": [
//         {
//             "id": 13,
//             "realtime_start": "2013-08-14",
//             "realtime_end": "2013-08-14",
//             "name": "G.17 Industrial Production and Capacity Utilization",
//             "press_release": true,
//             "link": "http://www.federalreserve.gov/releases/g17/"
//         },
#[derive(Deserialize, Debug)]
pub struct Releases {
    pub releases: Vec<Source>,
}

#[derive(Debug, Deserialize)]
pub struct Observation {
    pub realtime_start: String,
    pub realtime_end: String,
    pub date: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Observations {
    #[serde(rename = "observations")]
    pub inner: Vec<Observation>,
}
