use anyhow::Error;
use crate::Atomic;
use serde_json::Value;
// pub use quickwit::{Quickwit, QuickwitAtom};

pub trait ReprAtomic {
    fn repr(&self) ->Result<String, Error> ;
}
pub trait Store {
    fn store_object(index: &str, input: impl ReprAtomic) -> impl Future<Output = Result<(), Error>>;
}

pub trait Search {
    fn get_objects(index: &str, query: &str) -> impl Future<Output = Result<Value, Error>>;
}

pub mod quickwit;
pub mod opensearch;
