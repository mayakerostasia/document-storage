use clap::Args;

use super::ServiceType;

#[derive(Args, Debug)]
#[command(
    version,
    about = "Document Search",
    long_about = r#"Search database for documents"#
)]
pub struct Search {
    #[arg(short, long)]
    query: String,
    #[arg(short, long)]
    index: String,
    #[arg(short, long)]
    pub server: Option<ServiceType>,
}

impl Search {
    pub fn get_query(&self) -> String {
        self.query.clone()
    }

    pub fn get_index(&self) -> String {
        self.index.clone()
    }
}

// impl Display for Search {
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
