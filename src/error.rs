#[derive(thiserror::Error, Debug)]
pub enum Error {
   #[error("Not specified {0}")]
    NotSpecified(String), // 0 is the first argument
    #[error(transparent)]
    IO(#[from] std::io::Error),
}