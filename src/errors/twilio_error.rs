#[derive(Debug, PartialEq)]
pub enum TwilioError {
    UnknownError,
    IncorrectCode,
    TooManyRequests
}