use crate::{Atomic, cli::Upload};
use super::super::ReprAtomic;

pub struct QuickwitAtom {
    field: Option<String>,
    data: Atomic,
}

impl From<&Upload> for QuickwitAtom {
    fn from(value: &Upload) -> Self {
        QuickwitAtom { 
            field: value.field.clone(), 
            data: value.get_data().into()
        }
    }
}

impl ReprAtomic for QuickwitAtom {
    fn repr(&self) ->Result<String, anyhow::Error> {
        let mut document_buffer = String::new();
        let atom_data = self.data.get_data();

        let data_value = match &self.field {
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
            document_buffer.push_str(&serde_json::to_string(val).unwrap());
            document_buffer.push('\n');
        };
        Ok(document_buffer)
    }
}

// impl Display for Upload {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let data_value = match &self.field {
//             Some(fld) => {
//                 match self.data.get(fld) {
//                     Some(val) => val.clone(),
//                     None => panic!("Field ({:?}) not found in data, or isn't an array", fld)
//                 }
//             },
//             None => {
//                 assert!(self.data.is_array());
//                 self.data.clone().into_inner()
//             }
//         };
//         let data_array = data_value.as_array().unwrap();
//         for val in data_array {
//             f.write_str(&serde_json::to_string(val).unwrap()).unwrap();
//             f.write_str("\n").unwrap();
//         };
//         Ok(())
//     }
// }
