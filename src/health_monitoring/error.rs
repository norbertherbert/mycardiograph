use derive_more::{From, Display};


pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug, From, Display)]
pub enum Error {

    #[from]
    Custom(String),

}

impl std::error::Error for Error {}
