#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectEndOfBuff,
    Other(String),
}


pub type Result<T> = std::result::Result<T, Error>;