use super::super::{ReprAtomic, Search, Store};
use anyhow::Error;
use reqwest::header::HeaderMap;
use serde_json::Value;

const OPENSEARCH_URL: &str = "https://127.0.0.1:9200/{INDEX}/_bulk";

pub struct OpenSearchService;

impl Store for OpenSearchService {
    async fn store_object(index: &str, input: impl ReprAtomic) -> Result<(), Error> {
        let url = OPENSEARCH_URL.replace("{INDEX}", index);

        let mut headers = HeaderMap::new();
        headers.insert("content-type", "application/json".parse().unwrap());

        let client = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .default_headers(headers)
            .build()?;

        let response = client
            .post(url)
            .body(input.repr()?)
            .basic_auth("admin", Some("MKroot098"))
            .send()
            .await?;

        eprintln!("Response from OS: {:#?}", response);
        eprintln!("Response from OS: {:#?}", response.text().await?);

        Ok(())
    }
}

impl Search for OpenSearchService {
    async fn get_objects(index: &str, query: &str) -> Result<Value, Error> {
        let url = OPENSEARCH_URL.replace("{INDEX}", index) + "?" + query;
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
        let res = OpenSearchService::get_objects("mk-test-1", "query=id:2731922").await?;
        eprintln!("{}", res);
        Ok(())
    }
}
