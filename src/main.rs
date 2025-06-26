use anyhow::Error;
use clap::Parser;
use document_storage::{
    cli::{Commands, QuickwitUploader, ServiceType, Upload},
    storage::{opensearch::OpenSearch, quickwit::{Quickwit, QuickwitAtom}, Search, Store},
};

async fn quickwit_upload(upload: Upload) -> Result<(), Error> {
    let input_typed = QuickwitAtom::from(&upload);
    Quickwit::store_object(&upload.get_index(), input_typed).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let opts: QuickwitUploader = QuickwitUploader::parse();
    match opts.command {
        Some(Commands::Upload(upload)) => {
            // eprintln!("{}", upload.to_string());
            match upload.server {
                Some(ServiceType::Opensearch) => {
                    todo!();
                    // OpenSearch::store_object(&upload.get_index(), upload.into()).await?
                },
                Some(ServiceType::Quickwit) => {
                    quickwit_upload(upload).await?
                },
                None => {
                    quickwit_upload(upload).await?
                }
            }
        }
        Some(Commands::Search(_search)) => {
            unimplemented!()
        }
        None => {
            eprintln!("No command provided");
        }
    };

    Ok(())
}
