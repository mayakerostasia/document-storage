use serde_json::Value;
use anyhow::Error;
use crate::Atomic;
use super::{ReprAtomic, Search, Store};

const OPENSEARCH_URL: &str = "http://127.0.0.1:9200/{INDEX}/_doc/{ID}";

pub struct OpenSearch ;

impl Store for OpenSearch {
    async fn store_object(index: &str, input: impl ReprAtomic) -> Result<(), Error> {
        let url = OPENSEARCH_URL.replace("{INDEX}", index);
        let client = reqwest::Client::new();
        let response = client.post(url)
            .body(input.repr()?)
            .send()
            .await?;

        eprintln!("Response from QW: {:#?}", response);

        Ok(())
    }
}

impl Search for OpenSearch {
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
        let res = OpenSearch::get_objects("mk-test-1", "query=id:2731922").await?;
        eprintln!("{}", res);
        Ok(())
    }
}
