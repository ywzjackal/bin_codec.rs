#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectEndOfBuff,
}


pub type Result<T> = core::result::Result<T, Error>;