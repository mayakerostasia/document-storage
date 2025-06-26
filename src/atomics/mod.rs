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
    pub fn get_data(&self) -> Value {
        self.data.0.clone()
    }

    pub fn from_array_in_object(_array_field: &str, _value: Value) -> Self {
        todo!()
    }

    pub fn basic_value(value: Value) -> Self {
        Atomic {
            id: IdQuark("".to_string()),
            meta: MetaQuark(json!({})),
            data: DataQuark(value)
        }
    }
}

impl From<Value> for Atomic {
    fn from(value: Value) -> Self {
        Atomic::basic_value(value)
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
