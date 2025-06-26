use std::fmt::Display;
use serde_json::{Value, json};
use serde::{Serialize, Deserialize};
use crate::cli::Upload;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Atomic {
    id: IdQuark,
    meta: MetaQuark,
    data: DataQuark, 
}

impl Atomic {
    pub fn from_array_in_object(array_field: &str, value: Value) -> Self {
        todo!()
    }

    pub fn from_data(id: &str, value: Value) -> Self {
        Atomic {
            id: IdQuark(id.to_string()),
            meta: MetaQuark(json!({})),
            data: DataQuark(value)
        }
    }
}

impl From<Value> for Atomic {
    fn from(value: Value) -> Self {
        let id = value.get("id").expect("No 'id' key in object").to_string();
        Atomic::from_data(&id ,value)
    }

}
impl From<Upload> for Atomic {
    fn from(value: Upload) -> Self {
        let data = value.get_data();
        data.into()
    }
    
}
impl Display for Atomic {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&serde_json::to_string(self).expect("Failed to stringify Atomic"))
    }
}

// impl Store for Atomic {
//     async fn store_object(index: String, input: Atomic) -> Result<(), Error> {
//         todo!()
//     }
// }
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdQuark(String);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataQuark(Value);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaQuark(Value);
