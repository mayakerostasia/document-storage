use serde::{Deserialize, Serialize};

use super::super::ReprAtomic;
use crate::{Atomic, cli::Upload};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpensearchAtom {
    index: String,
    id_field: Option<String>,
    array_field: Option<String>,
    action: OpenSearchActionType,
    data: Atomic,
}

impl OpensearchAtom {
    fn new(
        index: &str,
        id_field: Option<String>,
        array_field: Option<String>,
        action: OpenSearchActionType,
        atom: Atomic,
    ) -> Self {
        OpensearchAtom {
            index: index.to_string(),
            id_field,
            array_field,
            action,
            data: atom,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OpenSearchActionType {
    Create,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
enum OpenSearchAction {
    #[serde(rename = "create")]
    Create(OpensearchMeta),
    #[serde(rename = "index")]
    Index(OpensearchMeta),
}

impl OpenSearchAction {
    pub fn create(index: &str, id: &str) -> Self {
        OpenSearchAction::Create(OpensearchMeta {
            _id: id.to_string(),
            _index: index.to_string(),
        })
    }

    pub fn index(index: &str, id: &str) -> Self {
        OpenSearchAction::Index(OpensearchMeta {
            _id: id.to_string(),
            _index: index.to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpensearchMeta {
    pub _id: String,
    pub _index: String,
}

impl From<&Upload> for OpensearchAtom {
    fn from(value: &Upload) -> Self {
        OpensearchAtom {
            index: value.get_index(),
            id_field: value.identity_field.clone(),
            array_field: value.array_field.clone(),
            action: OpenSearchActionType::Create,
            data: value.get_data().into(),
        }
    }
}

impl ReprAtomic for OpensearchAtom {
    fn repr(&self) -> Result<String, anyhow::Error> {
        let mut document_buffer = String::new();
        let atom_data = self.data.get_data();

        let data_value = match &self.array_field {
            Some(fld) => match atom_data.get(fld) {
                Some(val) => val.clone(),
                None => panic!("Field ({:?}) not found in data, or isn't an array", fld),
            },
            None => {
                assert!(atom_data.is_array());
                atom_data
            }
        };
        let data_array = data_value.as_array().unwrap();

        for val in data_array {
            let id_value = match &self.id_field {
                Some(fld) => {
                    let field_value = val.get(fld.clone()).expect("No key named {fld}");
                    field_value.as_str().unwrap()
                }
                None => {
                    panic!("ID field not present in array member")
                }
            };

            document_buffer.push_str(&serde_json::to_string(&OpenSearchAction::index(
                &self.index,
                id_value,
            ))?);
            document_buffer.push('\n');
            document_buffer.push_str(&serde_json::to_string(val).unwrap());
            document_buffer.push('\n');
        }
        Ok(document_buffer)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Error;
    use serde_json::json;

    #[test]
    fn test_opensearch_repr() -> Result<(), Error> {
        let atom = OpensearchAtom::new(
            "mk-test-1",
            Some("id".to_string()),
            Some("result".to_string()),
            OpenSearchActionType::Create,
            Atomic::basic_value(json!({
                "result": [
                  {
                    "id": "123",
                    "hello": "world",
                  }
                ]
            })),
        );
        eprintln!("{}", atom.repr()?);
        Ok(())
    }
}
