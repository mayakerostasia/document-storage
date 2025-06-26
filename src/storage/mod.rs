use anyhow::Error;
use serde_json::Value;

pub trait ReprAtomic {
    fn repr(&self) -> Result<String, Error>;
}
pub trait Store {
    fn store_object(index: &str, input: impl ReprAtomic)
    -> impl Future<Output = Result<(), Error>>;
}

pub trait Search {
    fn get_objects(index: &str, query: &str) -> impl Future<Output = Result<Value, Error>>;
}

pub mod opensearch;
pub mod quickwit;
