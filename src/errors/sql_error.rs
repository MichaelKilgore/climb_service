#[derive(Debug, PartialEq)]
pub enum SqlError {
    PrimaryKeyAlreadyExists,
    UnknownError,
    ConnectionError(String)
}