mod schema;

use dotenv::{dotenv, var};
use serde_json::Value;

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

// API Documentation:
//      >> "https://docs.kraken.com/api/docs/guides/spot-rest-auth"
//      >> "https://docs.kraken.com/api/docs/rest-api/get-server-time"
kvapi::api! {
    name:       Kraken
    base:       "https://api.kraken.com/0/"
    headers:    {
                    // API-Key HTTP header parameter: the public key from your API key-pair
                    "API-Key": &var("KRAKEN_API").expect("failed to find KRAKEN_API env. var.")

                    // API-Sign HTTP header parameter: encrypted signature of message.

                    // nonce payload parameter: always increasing, unsigned 64-bit integer.

                    // otp payload parameter: one-time-password and is only required if additional 2FA is configured for API.

                }

    dict:       {
                    #[rename: "time"] "public/Time": Value,
                }
}

/////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let kraken = Kraken::new();
    println!("{:#?}", kraken.time.get().await?);

    Ok(())
}
