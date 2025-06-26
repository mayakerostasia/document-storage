use anyhow::Error;
use clap::Parser;
use document_storage::{
    cli::{Commands, QuickwitUploader, ServiceType},
    storage::{OpenSearch, Search, Store, Quickwit},
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: QuickwitUploader = QuickwitUploader::parse();
    match opts.command {
        Some(Commands::Upload(upload)) => {
            // eprintln!("{}", upload.to_string());
            match upload.server {
                Some(ServiceType::Opensearch) => {
                    OpenSearch::store_object(&upload.get_index(), upload.into()).await?
                },
                Some(ServiceType::Quickwit) => {
                    Quickwit::store_object(&upload.get_index(), upload.into()).await?
                },
                None => {
                    Quickwit::store_object(&upload.get_index(), upload.into()).await?
                }
            }
        }
        Some(Commands::Search(search)) => {
            unimplemented!()
        }
        None => {
            eprintln!("No command provided");
        }
    };

    Ok(())
}
