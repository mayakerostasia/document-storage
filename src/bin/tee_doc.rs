use anyhow::Error;
use clap::Parser;
use document_storage::{
    cli::{Commands, QuickwitUploader, ServiceType, Upload},
    storage::{
        Search, Store,
        opensearch::{OpenSearchService, OpensearchAtom},
        quickwit::{Quickwit, QuickwitAtom},
    },
};

async fn quickwit_upload(upload: &Upload) -> Result<(), Error> {
    let input_typed = QuickwitAtom::from(upload);
    Quickwit::store_object(&upload.get_index(), input_typed).await?;
    Ok(())
}

async fn opensearch_upload(upload: &Upload) -> Result<(), Error> {
    let atomic = OpensearchAtom::from(upload);
    OpenSearchService::store_object(&upload.get_index(), atomic).await?;
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
                    opensearch_upload(&upload).await?;
                }
                Some(ServiceType::Quickwit) => quickwit_upload(&upload).await?,
                None => quickwit_upload(&upload).await?,
            }
            if upload.pipe {
                let _ = serde_json::to_writer_pretty(std::io::stdout(), &upload.get_data());
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
