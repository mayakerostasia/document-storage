use anyhow::Error;
use crate::Atomic;
use serde_json::Value;
pub use opensearch::OpenSearch;
pub use quickwit::Quickwit;

pub trait Store {
    fn store_object(index: &str, input: Atomic) -> impl Future<Output = Result<(), Error>>;
}

pub trait Search {
    fn get_objects(index: &str, query: &str) -> impl Future<Output = Result<Value, Error>>;
}

mod quickwit;
mod opensearch;
