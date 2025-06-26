use serde::{Deserialize, Serialize};

use crate::{Atomic, cli::Upload};
use super::super::ReprAtomic;

#[derive(Debug, Clone)]
pub struct OpensearchAtom {
    index: String,
    id_field: Option<String>,
    array_field: Option<String>,
    data: Atomic,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct OpensearchMeta {

}

impl OpensearchAtom {
    fn repr_meta(&self) -> {

    }
}

impl From<&Upload> for OpensearchAtom {
    fn from(value: &Upload) -> Self {
        OpensearchAtom {
            index: value.get_index(),
            id_field: value.identity_field.clone(),
            array_field: value.array_field.clone(),
            data: value.get_data().into()
        }
    }
}

impl ReprAtomic for OpensearchAtom {
    fn repr(&self) ->Result<String, anyhow::Error> {
        let mut document_buffer = String::new();
        let atom_data = self.data.get_data();

        let data_value = match &self.array_field {
            Some(fld) => {
                match atom_data.get(fld) {
                    Some(val) => val.clone(),
                    None => panic!("Field ({:?}) not found in data, or isn't an array", fld)
                }
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
                    fld.clone()
                },
                None => {
                    panic!("ID field not present in array member")
                },
            };

            document_buffer.push_str()
            document_buffer.push_str(&serde_json::to_string(val).unwrap());
            document_buffer.push('\n');
        };
        Ok(document_buffer)
    }
}
