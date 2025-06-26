use thiserror::Error;

#[derive(Error, Debug)]
pub enum AtomicError {
    #[error("Atomic Error")]
    FailureToDisplay(#[from] serde_json::Error),
}
