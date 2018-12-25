#[derive(Debug, PartialEq)]
pub enum Error {
    UnexpectEob { offset: usize, expected: usize, real: usize },
    Other(String),
}


pub type Result<T> = std::result::Result<T, Error>;