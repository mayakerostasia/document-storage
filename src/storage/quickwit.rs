use anyhow::Error;
use serde_json::Value;

use super::{Search, Store};

const STORAGE_URL: &str = "http://127.0.0.1:7280/api/v1/{INDEX}/ingest?commit=force";
const FETCH_URL: &str = "http://127.0.0.1:7280/api/v1/{INDEX}/search";

pub struct Quickwit;

impl Store for Quickwit {
     async fn store_object(index: &str, input: crate::Atomic) -> Result<(), Error> {
        let url = STORAGE_URL.replace("{INDEX}", index);
        let client = reqwest::Client::new();
        let response = client.post(url)
            .body(format!("{}", input))
            .send()
            .await?;

        eprintln!("Response from QW: {:#?}", response);

        Ok(())
    }
}

impl Search for Quickwit {
    async fn get_objects(index: &str, query: &str) -> Result<Value, Error> {
        let url = FETCH_URL.replace("{INDEX}", index) + "?" + query;
        let client = reqwest::Client::new();
        let response = client.get(url).send().await?;
        eprintln!("Response from QW: {:#?}", response);
        Ok(response.json().await?)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_object() -> Result<(), Error> {
        let res = Quickwit::get_objects("mk-test-1", "query=id:2731922").await?;
        eprintln!("{}", res);
        Ok(())
    }
}
