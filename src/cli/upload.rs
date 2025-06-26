use std::fmt::Display;
use clap::Args;
use clap_stdin::MaybeStdin;
use serde_json::Value;

use super::ServiceType;

#[derive(Args, Debug)]
#[command(
    version,
    about = "Quickwit Upload",
    long_about = r#"Uploads data to quickwit"#
)]
pub struct Upload {
    #[arg(short, long)]
    data: MaybeStdin<Value>,
    #[arg(short, long)]
    pub array_field: Option<String>,
    #[arg(short, long)]
    index: String,
    #[arg(long("id"))]
    pub identity_field: Option<String>,
    #[arg(short, long)]
    pub server: Option<ServiceType>
}

impl Upload {
    pub fn get_data(&self) -> Value {
        self.data.clone().into_inner()
    }

    pub fn get_index(&self) -> String {
        self.index.clone()
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
